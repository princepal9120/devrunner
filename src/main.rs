use clap::{CommandFactory, Parser};
use clap_complete::generate;
use devrunner::cli::{Cli, Commands};
use devrunner::config::Config;
use devrunner::error::exit_codes;
use devrunner::output;
use devrunner::runner::{check_conflicts, execute, search_runners};
use devrunner::scripts;
use devrunner::update;
use std::env;
use std::io;
use std::process;

fn main() {
    // Check for internal update flag (used by background updater)
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--internal-update-check" {
        // Run update check in background
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let _ = rt.block_on(update::perform_update_check());
        return;
    }

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let config = Config::load();

    // Merge config with CLI arguments
    let verbose = cli.verbose || config.get_verbose();
    let quiet = cli.quiet || config.get_quiet();
    let max_levels = cli.levels;
    let mut ignore_list = config.ignore_tools.clone();
    ignore_list.extend(cli.ignore.clone());

    // Check for update notification
    update::check_update_notification(quiet);

    // Handle subcommands
    match &cli.subcommand {
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, name, &mut io::stdout());
            return;
        }
        Some(Commands::List) => {
            handle_list_command(&ignore_list, max_levels, verbose);
            return;
        }
        Some(Commands::Why) => {
            handle_why_command(&ignore_list, max_levels, verbose);
            return;
        }
        Some(Commands::Doctor) => {
            handle_doctor_command(&ignore_list, max_levels);
            return;
        }
        None => {}
    }

    // Handle --update flag
    if cli.update {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        match rt.block_on(update::perform_blocking_update(quiet)) {
            Ok(_) => process::exit(exit_codes::SUCCESS),
            Err(e) => {
                output::error(&format!("Update failed: {}", e));
                process::exit(exit_codes::GENERIC_ERROR);
            }
        }
    }

    // Require a command
    let command = match &cli.command {
        Some(cmd) => cmd.clone(),
        None => {
            // If no command, just show help
            Cli::command().print_help().unwrap();
            println!();
            process::exit(exit_codes::SUCCESS);
        }
    };

    // Resolve alias (e.g., "t" -> "test")
    let command = config.resolve_alias(&command);

    // Get current directory
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            output::error(&format!("Failed to get current directory: {}", e));
            process::exit(exit_codes::GENERIC_ERROR);
        }
    };

    // Search for runners
    let (runners, working_dir) = match search_runners(
        &current_dir,
        max_levels,
        &ignore_list,
        verbose,
    ) {
        Ok(result) => result,
        Err(e) => {
            output::error(&e.to_string());
            eprintln!("Hint: Use --levels=N to increase search depth or check if you're in the right directory.");
            process::exit(e.exit_code());
        }
    };

    // Check for conflicts and select runner
    let runner = match check_conflicts(&runners, verbose) {
        Ok(r) => r,
        Err(e) => {
            output::error(&e.to_string());
            process::exit(e.exit_code());
        }
    };

    // Check if script exists and suggest alternatives if not (for Node.js projects)
    if runner.ecosystem == devrunner::detectors::Ecosystem::NodeJs {
        if let Some(script_list) = scripts::get_scripts_for_runner(&runner, &working_dir) {
            let script_names: Vec<String> = script_list.scripts.iter().map(|s| s.name.clone()).collect();
            
            if !devrunner::fuzzy::is_exact_match(&command, &script_names) {
                use owo_colors::OwoColorize;
                
                output::error(&format!("Script \"{}\" not found", command));
                println!();
                println!("{}", format!("Available scripts: {}", script_names.join(", ")).dimmed());
                
                if let Some(suggestion) = devrunner::fuzzy::suggest_script(&command, &script_names) {
                    println!();
                    println!("💡 Did you mean: {} {}", "devrunner".cyan(), suggestion.green().bold());
                }
                process::exit(exit_codes::GENERIC_ERROR);
            }
        }
    }

    // Record start time for timing
    let start_time = std::time::Instant::now();

    // Execute the command
    let result = match execute(
        &runner,
        &command,
        &cli.args,
        &working_dir,
        cli.dry_run,
        verbose,
        quiet,
    ) {
        Ok(r) => r,
        Err(e) => {
            output::error(&e.to_string());
            process::exit(e.exit_code());
        }
    };

    // Show execution time if enabled
    if config.get_show_timing() && !quiet && !cli.dry_run {
        use owo_colors::OwoColorize;
        let elapsed = start_time.elapsed();
        let seconds = elapsed.as_secs_f64();
        
        if seconds < 60.0 {
            eprintln!("\n{} Completed in {:.2}s", "✓".green(), seconds);
        } else {
            let minutes = (seconds / 60.0).floor() as u64;
            let remaining_secs = seconds % 60.0;
            eprintln!("\n{} Completed in {}m {:.1}s", "✓".green(), minutes, remaining_secs);
        }
    }

    // For dry run, always exit successfully
    if cli.dry_run {
        process::exit(exit_codes::SUCCESS);
    }

    // Spawn background update check (after command completes)
    if config.get_auto_update() && !update::is_update_disabled() {
        update::spawn_background_update();
    }

    // Exit with the same code as the executed command
    let exit_code = result
        .exit_status
        .code()
        .unwrap_or(exit_codes::GENERIC_ERROR);
    process::exit(exit_code);
}

/// Handle the `list` subcommand - show available scripts
fn handle_list_command(ignore_list: &[String], max_levels: u8, verbose: bool) {
    use owo_colors::OwoColorize;

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            output::error(&format!("Failed to get current directory: {}", e));
            process::exit(exit_codes::GENERIC_ERROR);
        }
    };

    // Find the project directory
    let (runners, working_dir) = match search_runners(&current_dir, max_levels, ignore_list, verbose) {
        Ok(result) => result,
        Err(e) => {
            output::error(&e.to_string());
            process::exit(e.exit_code());
        }
    };

    if runners.is_empty() {
        output::error("No runner detected in this project");
        process::exit(exit_codes::RUNNER_NOT_FOUND);
    }

    let runner = &runners[0];
    println!("📦 Detected: {} ({})", runner.name.green(), runner.detected_file.dimmed());
    println!();

    // Get scripts for this runner
    if let Some(script_list) = scripts::get_scripts_for_runner(runner, &working_dir) {
        println!("{}", "Available scripts:".bold());
        
        // Find the longest script name for alignment
        let max_name_len = script_list.scripts.iter().map(|s| s.name.len()).max().unwrap_or(0);
        
        for script in &script_list.scripts {
            println!(
                "  {}{}  {}",
                script.name.cyan(),
                " ".repeat(max_name_len - script.name.len()),
                script.command.dimmed()
            );
        }
    } else {
        println!("{}", "No scripts found for this project type.".dimmed());
    }

    process::exit(exit_codes::SUCCESS);
}

/// Handle the `why` subcommand - explain runner selection
fn handle_why_command(ignore_list: &[String], max_levels: u8, _verbose: bool) {
    use devrunner::detectors::detect_all;
    use owo_colors::OwoColorize;

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            output::error(&format!("Failed to get current directory: {}", e));
            process::exit(exit_codes::GENERIC_ERROR);
        }
    };

    // Search for all runners (without ignoring for comparison)
    let mut search_dir = current_dir.clone();
    let mut found_level = 0;
    let mut all_runners = Vec::new();

    for level in 0..=max_levels {
        let runners = detect_all(&search_dir, &[]);
        if !runners.is_empty() {
            all_runners = runners;
            found_level = level;
            break;
        }
        if let Some(parent) = search_dir.parent() {
            search_dir = parent.to_path_buf();
        } else {
            break;
        }
    }

    if all_runners.is_empty() {
        output::error("No runner detected in this project");
        process::exit(exit_codes::RUNNER_NOT_FOUND);
    }

    // Get the selected runner (with ignore list applied)
    let filtered_runners: Vec<_> = all_runners
        .iter()
        .filter(|r| !ignore_list.iter().any(|i| i.eq_ignore_ascii_case(&r.name)))
        .collect();

    println!("{}", "Runner Selection Analysis".bold().underline());
    println!();

    if let Some(selected) = filtered_runners.first() {
        println!("📦 {} {}", "Using:".bold(), selected.name.green().bold());
        println!(
            "   {} Found {} in {} (level {})",
            "→".dimmed(),
            selected.detected_file.cyan(),
            search_dir.display(),
            found_level
        );
        println!(
            "   {} Priority: {} (lower = higher priority)",
            "→".dimmed(),
            selected.priority
        );
        println!();

        // Show other candidates
        if all_runners.len() > 1 {
            println!("{}", "Other detected runners:".bold());
            for runner in &all_runners {
                if runner.name != selected.name {
                    let status = if ignore_list.iter().any(|i| i.eq_ignore_ascii_case(&runner.name)) {
                        "(ignored via --ignore)".red().to_string()
                    } else {
                        format!("(priority {})", runner.priority).dimmed().to_string()
                    };
                    println!(
                        "  {} {} - {} {}",
                        "•".dimmed(),
                        runner.name,
                        runner.detected_file,
                        status
                    );
                }
            }
        }
    } else {
        println!("{}", "All detected runners were ignored!".red());
        println!();
        println!("{}", "Detected (but ignored):".bold());
        for runner in &all_runners {
            println!("  {} {} - {}", "•".dimmed(), runner.name, runner.detected_file);
        }
    }

    process::exit(exit_codes::SUCCESS);
}

/// Handle the `doctor` subcommand - diagnose project setup
fn handle_doctor_command(ignore_list: &[String], max_levels: u8) {
    use devrunner::detectors::{detect_all, is_tool_installed};
    use owo_colors::OwoColorize;

    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            output::error(&format!("Failed to get current directory: {}", e));
            process::exit(exit_codes::GENERIC_ERROR);
        }
    };

    println!("{}", "🩺 Devrunner Project Diagnosis".bold().underline());
    println!();

    // Find project directory
    let (runners, working_dir) = match search_runners(&current_dir, max_levels, ignore_list, false) {
        Ok(result) => result,
        Err(_) => {
            println!("{} No project detected", "❌".red());
            process::exit(exit_codes::RUNNER_NOT_FOUND);
        }
    };

    println!("{}", "Project Detection:".bold());
    println!("  {} Project root: {}", "→".dimmed(), working_dir.display());
    println!();

    // Check all runners and their tools
    println!("{}", "Detected Runners:".bold());
    let all_runners = detect_all(&working_dir, &[]);
    
    for runner in &all_runners {
        let installed = is_tool_installed(&runner.name);
        let status_text = if installed {
            let version = get_tool_version(&runner.name).unwrap_or_else(|| "installed".to_string());
            format!("{}", version.dimmed())
        } else {
            format!("{}", "not installed".red())
        };
        
        if installed {
            print!("  {} ", "✓".green());
        } else {
            print!("  {} ", "✗".red());
        }
        println!(
            "{} ({}) - {}",
            runner.name,
            runner.detected_file,
            status_text
        );
    }
    println!();

    // Check for conflicts
    let mut has_conflicts = false;
    let mut ecosystems: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    for runner in &all_runners {
        ecosystems
            .entry(runner.ecosystem.as_str().to_string())
            .or_default()
            .push(runner.name.clone());
    }

    println!("{}", "Conflict Analysis:".bold());
    for (ecosystem, tools) in &ecosystems {
        if tools.len() > 1 {
            has_conflicts = true;
            println!(
                "  {} {} ecosystem has multiple lockfiles: {}",
                "⚠".yellow(),
                ecosystem,
                tools.join(", ").yellow()
            );
        }
    }
    
    if !has_conflicts {
        println!("  {} No lockfile conflicts detected", "✓".green());
    }
    println!();

    // Script count
    if let Some(script_list) = scripts::get_scripts_for_runner(&runners[0], &working_dir) {
        println!(
            "{} {} scripts available in {}",
            "✓".green(),
            script_list.scripts.len(),
            script_list.source_file
        );
    }

    process::exit(exit_codes::SUCCESS);
}

/// Try to get the version of a tool
fn get_tool_version(tool: &str) -> Option<String> {
    use std::process::Command;

    let version_flag = match tool {
        "npm" | "pnpm" | "yarn" | "bun" => "-v",
        "cargo" | "rustc" => "--version",
        "python" | "python3" => "--version",
        "go" => "version",
        "node" => "-v",
        _ => "--version",
    };

    let output = Command::new(tool)
        .arg(version_flag)
        .output()
        .ok()?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        let version = version.trim();
        // Extract just the version number if possible
        let version = version
            .split_whitespace()
            .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
            .unwrap_or(version);
        Some(version.trim_start_matches('v').to_string())
    } else {
        None
    }
}
