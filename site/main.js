/* Premium Dynamic Interactivity for Devrunner Homepage and Docs */

// 1. Theme Management (Sleek Transition Safeguard)
function toggleTheme() {
    const html = document.documentElement;
    const currentTheme = html.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    
    html.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcons(newTheme);
}

function updateThemeIcons(theme) {
    const moons = document.querySelectorAll('.moon-icon');
    const suns = document.querySelectorAll('.sun-icon');
    
    if (theme === 'dark') {
        moons.forEach(el => el.style.display = 'none');
        suns.forEach(el => el.style.display = 'block');
    } else {
        moons.forEach(el => el.style.display = 'block');
        suns.forEach(el => el.style.display = 'none');
    }
}

// 2. Interactive Terminal Simulator
const terminalData = {
    npm: {
        command: "dr dev",
        output: [
            "<span class='output-success'>✔ Detected Node.js (package.json)</span>",
            "<span class='output-info'>ℹ Running: npm run dev</span>",
            "",
            "<span style='color: #888'>> next dev</span>",
            "  ▲ Next.js 14.1.0",
            "  - Local:        http://localhost:3000",
            "  - Environments: .env.local",
            "",
            "✓ Ready in 1.2s (compiled successfully)"
        ]
    },
    cargo: {
        command: "dr test",
        output: [
            "<span class='output-success'>✔ Detected Rust (Cargo.toml)</span>",
            "<span class='output-info'>ℹ Running: cargo test</span>",
            "",
            "   Compiling devrunner v0.2.0 (/Users/project)",
            "    Finished test [unoptimized + debuginfo] target(s) in 0.45s",
            "     Running unittests src/lib.rs (target/debug/deps/devrunner-f21ae4)",
            "",
            "running 8 tests",
            "test tests::detect_bun_lockfile ... <span class='output-success'>ok</span>",
            "test tests::detect_cargo_lockfile ... <span class='output-success'>ok</span>",
            "test tests::resolve_monorepo_nx ... <span class='output-success'>ok</span>",
            "test tests::conflict_resolution ... <span class='output-success'>ok</span>",
            "",
            "test result: <span class='output-success'>ok</span>. 8 passed; 0 failed; 0 ignored; 0 measured"
        ]
    },
    python: {
        command: "dr start",
        output: [
            "<span class='output-success'>✔ Detected Python (poetry.lock)</span>",
            "<span class='output-info'>ℹ Running: poetry run python main.py</span>",
            "",
            "Starting application server...",
            "Loading variables from poetry environment.",
            "Uvicorn running on http://127.0.0.1:8000 (Press CTRL+C to quit)"
        ]
    },
    go: {
        command: "dr build",
        output: [
            "<span class='output-success'>✔ Detected Go (go.mod)</span>",
            "<span class='output-info'>ℹ Running: go build -o bin/main cmd/main.go</span>",
            "",
            "Building binary module 'github.com/princepal9120/devrunner'...",
            "Checking signatures...",
            "<span class='output-success'>✔ Finished compilation successfully in 120ms</span>"
        ]
    }
};

let terminalInterval = null;

function switchTerminalTab(tabName, element) {
    // Clear active states
    document.querySelectorAll('.terminal-tab').forEach(t => t.classList.remove('active'));
    element.classList.add('active');
    
    // Animate command and output
    const data = terminalData[tabName];
    if (!data) return;
    
    const cmdElement = document.getElementById('terminal-cmd');
    const outputElement = document.getElementById('terminal-output');
    
    if (terminalInterval) clearInterval(terminalInterval);
    
    // Reset terminal content
    cmdElement.innerHTML = '';
    outputElement.innerHTML = '';
    
    // Animate typing
    let charIndex = 0;
    const commandText = data.command;
    
    terminalInterval = setInterval(() => {
        if (charIndex < commandText.length) {
            cmdElement.innerHTML += commandText.charAt(charIndex);
            charIndex++;
        } else {
            clearInterval(terminalInterval);
            // Simulate typing delay before output shows up
            setTimeout(() => {
                outputElement.innerHTML = data.output.join('<br>');
            }, 200);
        }
    }, 45);
}

// 3. Interactive Tree Explorer (Path Traversal Visualizer)
function selectTreeNode(nodeName, element) {
    // Update active state in tree sidebar
    document.querySelectorAll('.tree-node').forEach(n => n.classList.remove('active'));
    element.classList.add('active');
    
    // Identify node behavior
    const steps = document.querySelectorAll('.flow-step');
    steps.forEach(s => s.classList.remove('active'));
    
    const detectStep = document.getElementById('step-detect');
    const resolveStep = document.getElementById('step-resolve');
    const runStep = document.getElementById('step-run');
    
    if (nodeName === 'components') {
        // Highlighting upward traversal
        setTimeout(() => {
            document.getElementById('step-traverse').classList.add('active');
            document.getElementById('step-traverse-desc').innerHTML = "Traversing up: <code>src/components</code> → <code>src</code> → <code>root</code>";
        }, 100);
        
        setTimeout(() => {
            detectStep.classList.add('active');
            detectStep.querySelector('.flow-text').innerHTML = "Detected lockfile: <code>package-lock.json</code> at root";
        }, 600);
        
        setTimeout(() => {
            runStep.classList.add('active');
            runStep.querySelector('.flow-text').innerHTML = "Executed: <code>npm run test</code>";
        }, 1100);
    } else if (nodeName === 'cargo') {
        // Rust context detection
        setTimeout(() => {
            document.getElementById('step-traverse').classList.add('active');
            document.getElementById('step-traverse-desc').innerHTML = "Traversing up: <code>root</code> (already at root)";
        }, 100);
        
        setTimeout(() => {
            detectStep.classList.add('active');
            detectStep.querySelector('.flow-text').innerHTML = "Detected manifest: <code>Cargo.toml</code> at root";
        }, 600);
        
        setTimeout(() => {
            runStep.classList.add('active');
            runStep.querySelector('.flow-text').innerHTML = "Executed: <code>cargo test</code>";
        }, 1100);
    } else if (nodeName === 'conflict') {
        // Conflict Scenario
        setTimeout(() => {
            document.getElementById('step-traverse').classList.add('active');
            document.getElementById('step-traverse-desc').innerHTML = "Traversing up: <code>root</code>";
        }, 100);
        
        setTimeout(() => {
            detectStep.classList.add('active');
            detectStep.querySelector('.flow-text').innerHTML = "Detected multiple lockfiles: <code>yarn.lock</code> AND <code>pnpm-lock.yaml</code>";
        }, 600);
        
        setTimeout(() => {
            resolveStep.classList.add('active');
            resolveStep.querySelector('.flow-text').innerHTML = "Consulted <code>package.json</code>'s <code>packageManager</code> field → Resolving to Yarn";
        }, 1100);
        
        setTimeout(() => {
            runStep.classList.add('active');
            runStep.querySelector('.flow-text').innerHTML = "Executed: <code>yarn test</code> with safe Corepack overrides";
        }, 1600);
    }
}

// 4. Docs Copy Command Switcher
const docsCommands = {
    mac: 'curl -fsSL install.cat/princepal9120/devrunner | bash',
    win: 'irm install.cat/princepal9120/devrunner | iex',
    cargo: 'cargo install devrunner-cli',
    npx: 'npx skills add princepal9120/devrunner'
};

function switchDocsInstallTab(os, element) {
    document.querySelectorAll('.install-tab-btn').forEach(btn => btn.classList.remove('active'));
    element.classList.add('active');
    
    const payloadText = document.getElementById('install-cmd-payload');
    if (payloadText && docsCommands[os]) {
        payloadText.innerText = docsCommands[os];
    }
}

function copyDocsCommand() {
    const payloadText = document.getElementById('install-cmd-payload').innerText;
    navigator.clipboard.writeText(payloadText).then(() => {
        const copyBtn = document.querySelector('.install-copy-btn');
        const original = copyBtn.innerHTML;
        
        copyBtn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-check"><path d="M20 6 9 17l-5-5"/></svg>';
        
        setTimeout(() => {
            copyBtn.innerHTML = original;
        }, 2000);
    });
}

// 5. Mobile Navbar Trigger
function toggleMobileMenu() {
    const sidebar = document.querySelector('.docs-sidebar');
    if (sidebar) {
        sidebar.classList.toggle('mobile-open');
    }
}

// 6. Startup Animations & Interactivity Initializers
document.addEventListener('DOMContentLoaded', () => {
    // A. Animate Performance Comparison Bars on Visible
    const perfSection = document.querySelector('.bento-grid');
    if (perfSection) {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const bars = document.querySelectorAll('.perf-bar');
                    bars.forEach(bar => {
                        const width = bar.getAttribute('data-width');
                        bar.style.width = width;
                    });
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.15 });
        observer.observe(perfSection);
    }
    
    // B. Trigger first terminal tab typing animation
    const firstTab = document.querySelector('.terminal-tab');
    if (firstTab) {
        switchTerminalTab('npm', firstTab);
    }
    
    // C. Trigger first tree node visualizer state
    const firstTreeNode = document.querySelector('.tree-node');
    if (firstTreeNode) {
        selectTreeNode('components', firstTreeNode);
    }
    
    // D. Sync Global Theme Selection
    const savedTheme = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', savedTheme);
    updateThemeIcons(savedTheme);
});
