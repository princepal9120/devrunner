
$ErrorActionPreference = "Stop"

$Repo = "princepal9120/devrunner"
$BinaryName = "devrunner.exe"
$InstallDir = "$env:USERPROFILE\.local\bin"

function Write-Info { param($Message) Write-Host "🔍 $Message" -ForegroundColor Cyan }
function Write-Success { param($Message) Write-Host "✓ $Message" -ForegroundColor Green }
function Write-Warning { param($Message) Write-Host "⚠ $Message" -ForegroundColor Yellow }
function Write-Error { param($Message) Write-Host "❌ $Message" -ForegroundColor Red }

function Get-Architecture {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    switch ($arch) {
        "X64" { return "x86_64" }
        "Arm64" { return "aarch64" }
        default { throw "Unsupported architecture: $arch" }
    }
}

function Get-LatestVersion {
    Write-Info "Fetching latest version..."
    $releaseUrl = "https://api.github.com/repos/$Repo/releases/latest"
    $release = Invoke-RestMethod -Uri $releaseUrl -Headers @{ "User-Agent" = "devrunner-installer" }
    return $release.tag_name
}

function Resolve-InstallDir {
    $userLocal = "$env:USERPROFILE\.local\bin"
    try {
        New-Item -ItemType Directory -Path $userLocal -Force | Out-Null
        return $userLocal
    } catch {}

    $programFilesDir = Join-Path $env:ProgramFiles "devrunner"
    try {
        New-Item -ItemType Directory -Path $programFilesDir -Force | Out-Null
        return $programFilesDir
    } catch {
        throw "Could not create install directory in $userLocal or $programFilesDir"
    }
}

function Install-Devrunner {
    # ASCII Art Banner
    Write-Host ""
    Write-Host "  ██████╗ ███████╗██╗   ██╗██████╗ ██╗   ██╗███╗   ██╗███╗   ██╗███████╗██████╗ " -ForegroundColor Blue
    Write-Host "  ██╔══██╗██╔════╝██║   ██║██╔══██╗██║   ██║████╗  ██║████╗  ██║██╔════╝██╔══██╗" -ForegroundColor Blue
    Write-Host "  ██║  ██║█████╗  ██║   ██║██████╔╝██║   ██║██╔██╗ ██║██╔██╗ ██║█████╗  ██████╔╝" -ForegroundColor Blue
    Write-Host "  ██║  ██║██╔══╝  ╚██╗ ██╔╝██╔══██╗██║   ██║██║╚██╗██║██║╚██╗██║██╔══╝  ██╔══██╗" -ForegroundColor Blue
    Write-Host "  ██████╔╝███████╗ ╚████╔╝ ██║  ██║╚██████╔╝██║ ╚████║██║ ╚████║███████╗██║  ██║" -ForegroundColor Blue
    Write-Host "  ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝" -ForegroundColor Blue
    Write-Host ""
    Write-Host "                   🚀 Universal Task Runner" -ForegroundColor Green
    Write-Host ""

    # Detect architecture
    $arch = Get-Architecture
    Write-Info "Detected architecture: windows-$arch"

    $InstallDir = Resolve-InstallDir
    Write-Info "Install directory: $InstallDir"

    # Get latest version
    $version = Get-LatestVersion
    Write-Info "Latest version: $version"

    # Build asset name and URL
    # Try devrunner first, then run
    $assetBase = "devrunner-windows-$arch.exe"
    $assetLegacy = "run-windows-$arch.exe"
    $downloadUrl = "https://github.com/$Repo/releases/download/$version/$assetBase"
    $checksumUrl = "$downloadUrl.sha256"

    # Check if devrunner asset exists
    try {
        $req = Invoke-WebRequest -Uri $downloadUrl -Method Head -ErrorAction Stop
    } catch {
        # Fallback
        $assetBase = $assetLegacy
        $downloadUrl = "https://github.com/$Repo/releases/download/$version/$assetBase"
        $checksumUrl = "$downloadUrl.sha256"
    }

    # Create temp directory
    $tempDir = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "devrunner-install-$(Get-Random)")
    $tempBinary = Join-Path $tempDir $BinaryName
    $tempChecksum = Join-Path $tempDir "$assetBase.sha256"

    try {
        # Download binary
        Write-Info "Downloading $assetBase..."
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempBinary -UseBasicParsing

        # Download and verify checksum
        Write-Info "Verifying checksum..."
        Invoke-WebRequest -Uri $checksumUrl -OutFile $tempChecksum -UseBasicParsing
        $checksumContent = (Get-Content $tempChecksum -Raw).Trim()
        $expectedHash = $checksumContent.Split()[0].ToUpper()
        if (-not $expectedHash -or $expectedHash.Length -ne 64) {
            throw "Invalid checksum file format for $assetBase"
        }
        $actualHash = (Get-FileHash -Path $tempBinary -Algorithm SHA256).Hash.ToUpper()

        if ($expectedHash -ne $actualHash) {
            throw "Checksum mismatch for $assetBase"
        }
        Write-Success "Checksum verified"

        # Create install directory
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
            Write-Info "Created directory: $InstallDir"
        }

        # Install binary
        $installPath = Join-Path $InstallDir $BinaryName
        Copy-Item -Path $tempBinary -Destination $installPath -Force
        Write-Success "Installed to $installPath"

        # Check if directory is in PATH
        $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($userPath -notlike "*$InstallDir*") {
            Write-Warning "$InstallDir is not in your PATH"
            Write-Host ""
            Write-Host "  To add it permanently, run:" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "    [Environment]::SetEnvironmentVariable('PATH', `$env:PATH + ';$InstallDir', 'User')" -ForegroundColor White
            Write-Host ""
            Write-Host "  Or add it to the current session:" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "    `$env:PATH += ';$InstallDir'" -ForegroundColor White
            Write-Host ""

            # Offer to add to PATH automatically
            $addToPath = Read-Host "  Add to PATH automatically? (y/N)"
            if ($addToPath -eq "y" -or $addToPath -eq "Y") {
                $newPath = "$userPath;$InstallDir"
                [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
                $env:PATH += ";$InstallDir"
                Write-Success "Added to PATH (restart terminal for changes to take effect)"
            }
        }

        Write-Host ""
        Write-Host "  ✅ Installation complete!" -ForegroundColor Green
        Write-Host ""
        Write-Host "  Run " -NoNewline; Write-Host "devrunner --help" -ForegroundColor Blue -NoNewline; Write-Host " to get started"
        Write-Host "  Example: " -NoNewline; Write-Host "devrunner test" -ForegroundColor Blue -NoNewline; Write-Host " or " -NoNewline; Write-Host "devrunner build" -ForegroundColor Blue
        Write-Host ""

    } finally {
        # Cleanup
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# Devrunner installer
Install-Devrunner
