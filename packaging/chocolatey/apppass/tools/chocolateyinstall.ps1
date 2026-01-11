$ErrorActionPreference = 'Stop'

$packageName = $env:ChocolateyPackageName
$toolsDir    = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64       = 'https://github.com/stescobedo92/apppass/releases/download/v1.0.0/apppass-x86_64-pc-windows-msvc.zip'
$checksum64  = 'f086e11f6ad4b3460f35d9c54d13aa42299abd7622b10d4dc739d16a77015c34'

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
}

# Download and extract the portable executable
Install-ChocolateyZipPackage @packageArgs

# Verify the executable exists
$exePath = Join-Path $toolsDir "apppass.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "apppass.exe not found at expected location: $exePath"
    throw "Installation failed: apppass.exe not found after extraction"
}

# Create shim for apppass.exe so it's available in PATH
# Install-BinFile is a Chocolatey function that creates a shim
Install-BinFile -Name "apppass" -Path $exePath
