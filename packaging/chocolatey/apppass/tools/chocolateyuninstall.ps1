$ErrorActionPreference = 'Stop'

$packageName = $env:ChocolateyPackageName
$toolsDir    = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove the shim
Uninstall-BinFile -Name "apppass"

# Remove the executable
$exePath = Join-Path $toolsDir "apppass.exe"
if (Test-Path $exePath) {
    Remove-Item $exePath -Force -ErrorAction SilentlyContinue
}

Write-Host "$packageName has been uninstalled."
