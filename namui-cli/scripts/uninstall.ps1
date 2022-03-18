function Main {
    $CargoBinDirPath = Get-CargoBinDirPath

    Remove-Symlink -CargoBinDirPath $CargoBinDirPath

    Remove-Config -CargoBinDirPath $CargoBinDirPath

    Write-Output "Successfully uninstalled."
}

# Error Code
$EXIT_SYMLINK_REMOVE_FAIL = 1
$EXIT_CONFIG_REMOVE_FAIL = 2

function Get-CargoBinDirPath {
    return "$Env:USERPROFILE\.cargo\bin"
}

function Remove-Symlink {
    param (
        [string] $CargoBinDirPath
    )

    $SymlinkPath = Join-Path -Path $CargoBinDirPath -ChildPath "\namui.exe"
    Remove-Item -Path $SymlinkPath
    if (!$?) {
        Write-Output "Could not remove symlink"
        Exit $EXIT_SYMLINK_REMOVE_FAIL
    }
}

function Remove-Config {
    param (
        [string] $CargoBinDirPath
    )

    $ConfigPath = Join-Path -Path $CargoBinDirPath -ChildPath "\namui.config.json"
    Remove-Item -Path $ConfigPath
    if (!$?) {
        Write-Output "Could not remove config"
        Exit $EXIT_CONFIG_REMOVE_FAIL
    }
}

Main
