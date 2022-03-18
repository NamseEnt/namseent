function Main {
    $CliRootPath = Get-CliRootPath
    $CliPath = Get-CliPath -CliRootPath $CliRootPath
    $CargoBinDirPath = Get-CargoBinDirPath

    Assert-CargoInstalled
    Assert-WasmPackInstalled
    Assert-CargoBinDirExist -CargoBinDirPath $CargoBinDirPath

    Build-Cli -CliRootPath $CliRootPath

    New-CliSymlink -CliPath $CliPath -CargoBinDirPath $CargoBinDirPath

    Initialize-Config -CliPath $CliPath -CargoBinDirPath $CargoBinDirPath

    Write-Output "Successfully installed."
}

# Error Code
$EXIT_CARGO_NOT_FOUND = 1
$EXIT_WASM_PACK_NOT_FOUND = 2
$EXIT_CARGO_BIN_DIR_NOT_FOUND = 3
$EXIT_CLI_BUILD_FAILED = 4
$EXIT_SYMLINK_MAKE_FAILED = 5
$EXIT_CONFIG_INIT_FAILED = 6

function Get-CliRootPath {
    return (Get-Item $PSScriptRoot).Parent.FullName
}

function Get-CliPath {
    param (
        [string] $CliRootPath
    )

    return Join-Path -Path $CliRootPath -ChildPath "\target\debug\namui-cli.exe"
}

function Get-CargoBinDirPath {
    return "$Env:USERPROFILE\.cargo\bin"
}

function Assert-CargoInstalled {
    cargo --version
    if (!$?) {
        Write-Output "Cargo command execution failed. Is there a cargo installed?"
        Exit $EXIT_CARGO_NOT_FOUND
    }
}

function Assert-WasmPackInstalled {
    wasm-pack --version
    if (!$?) {
        Write-Output "Wasm-pack command execution failed. Is there a wasm-pack installed?`n`If not, install it with https://rustwasm.github.io/wasm-pack/installer/"
        Exit $EXIT_WASM_PACK_NOT_FOUND
    }
}

function Assert-CargoBinDirExist {
    param (
        [string] $CargoBinDirPath
    )

    Test-Path $CargoBinDirPath
    if (!$?) {
        Write-Output "Could not find dir `"$CargoBinDirPath`". Is there a cargo installed?"
        Exit $EXIT_CARGO_BIN_DIR_NOT_FOUND
    }
}

function Build-Cli {
    param (
        [string] $CliRootPath
    )

    Set-Location -Path $CliRootPath
    cargo build
    if (!$?) {
        Write-Output "Cli build failed."
        Exit $EXIT_CLI_BUILD_FAILED
    }
}

function New-CliSymlink {
    param (
        [string] $CliPath,
        [string] $CargoBinDirPath
    )

    $SymlinkPath = Join-Path -Path $CargoBinDirPath -ChildPath "\namui.exe"
    New-item -ItemType SymbolicLink -Path $SymlinkPath -Target $CliPath -Force
    if (!$?) {
        Write-Output "Link failed."
        Exit $EXIT_SYMLINK_MAKE_FAILED
    }
}

function Initialize-Config {
    param (
        [string] $CliPath,
        [string] $CargoBinDirPath
    )

    $ConfigJson = @{ exe_path = $CliPath } | ConvertTo-Json
    $ConfigPath = Join-Path -Path $CargoBinDirPath -ChildPath "\namui.config.json"
    $Utf8NoBomEncoding = New-Object System.Text.UTF8Encoding $False
    [System.IO.File]::WriteAllLines($ConfigPath, $ConfigJson, $Utf8NoBomEncoding)
    if (!$?) {
        Write-Output "Config init failed"
        Exit $EXIT_CONFIG_INIT_FAILED
    }
}

Main
