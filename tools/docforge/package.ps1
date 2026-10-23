param(
    [string]$Proxy = "",
    [string]$MediaToolsDirectory = ""
)

$ErrorActionPreference = "Stop"

$toolRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$manifest = Join-Path $toolRoot "Cargo.toml"
$cargoHome = Join-Path $toolRoot "tmp\cargo-home"
$outRoot = Join-Path $toolRoot "out"
$outputRoot = Join-Path $outRoot "docforge-portable"
$archive = Join-Path $outRoot "docforge-portable-windows-x64.zip"
$targetRoot = Join-Path $toolRoot "target"
$binary = Join-Path $targetRoot "release\docforge.exe"

# All Cargo state is scoped to this tool directory for this process only.
$env:CARGO_HOME = $cargoHome
$env:CARGO_TARGET_DIR = $targetRoot
if ($Proxy) {
    $env:HTTP_PROXY = $Proxy
    $env:HTTPS_PROXY = $Proxy
}

& cargo build --release --locked --manifest-path $manifest
if ($LASTEXITCODE -ne 0) {
    throw "DocForge release build failed."
}
if (-not (Test-Path -LiteralPath $binary)) {
    throw "Release binary was not created: $binary"
}

New-Item -ItemType Directory -Force -Path $outRoot | Out-Null
if (Test-Path -LiteralPath $outputRoot) {
    $resolvedOutRoot = [System.IO.Path]::GetFullPath($outRoot).TrimEnd('\')
    $resolvedOutputRoot = [System.IO.Path]::GetFullPath($outputRoot).TrimEnd('\')
    if ((Split-Path -Parent $resolvedOutputRoot) -ne $resolvedOutRoot) {
        throw "Refusing to clean an unexpected output path: $resolvedOutputRoot"
    }
    Remove-Item -LiteralPath $resolvedOutputRoot -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null
Copy-Item -LiteralPath $binary -Destination (Join-Path $outputRoot "docforge.exe") -Force
Copy-Item -LiteralPath (Join-Path $toolRoot "README.zh-CN.txt") -Destination (Join-Path $outputRoot "README.zh-CN.txt") -Force

$fontSource = Join-Path $toolRoot "fonts"
if (Test-Path -LiteralPath $fontSource -PathType Container) {
    $fontTarget = Join-Path $outputRoot "fonts"
    New-Item -ItemType Directory -Force -Path $fontTarget | Out-Null
    Get-ChildItem -LiteralPath $fontSource -File |
        Copy-Item -Destination $fontTarget -Force
}

if ($MediaToolsDirectory) {
    if (-not (Test-Path -LiteralPath $MediaToolsDirectory -PathType Container)) {
        throw "MediaToolsDirectory does not exist: $MediaToolsDirectory"
    }
    $mediaTarget = Join-Path $outputRoot "bin"
    New-Item -ItemType Directory -Force -Path $mediaTarget | Out-Null
    Get-ChildItem -LiteralPath $MediaToolsDirectory -File |
        Copy-Item -Destination $mediaTarget -Force
}

Compress-Archive -LiteralPath $outputRoot -DestinationPath $archive -CompressionLevel Optimal -Force

Write-Host "Portable directory created: $outputRoot"
Write-Host "Portable archive created: $archive"
