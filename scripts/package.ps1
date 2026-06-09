param(
  [string]$OutputRoot = "D:\Work",
  [string]$AppName = "",
  [string]$Version = "",
  [switch]$Clean
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$targetDir = Join-Path $repoRoot "target"
$frontendDist = Join-Path $repoRoot "frontend\dist"
$bundleDir = Join-Path $repoRoot "target\release\bundle\msi"
$releaseDir = Join-Path $repoRoot "target\release"
$tauriConfigPath = Join-Path $repoRoot "src-tauri\tauri.conf.json"
$tauriCargoPath = Join-Path $repoRoot "src-tauri\Cargo.toml"
$tauriConfig = Get-Content -LiteralPath $tauriConfigPath -Raw | ConvertFrom-Json
$cargoToml = Get-Content -LiteralPath $tauriCargoPath -Raw
$cargoPackageName = if ($cargoToml -match '(?m)^\s*name\s*=\s*"([^"]+)"') {
  $Matches[1]
}
else {
  throw "Could not read package name from: $tauriCargoPath"
}

if ([string]::IsNullOrWhiteSpace($AppName)) {
  $AppName = $tauriConfig.productName
}
if ([string]::IsNullOrWhiteSpace($Version)) {
  $Version = $tauriConfig.version
}

function Remove-GeneratedPath {
  param([string]$Path)

  if (-not (Test-Path -LiteralPath $Path)) {
    return
  }

  $resolved = (Resolve-Path -LiteralPath $Path).Path
  if (-not ($resolved.StartsWith($repoRoot + [System.IO.Path]::DirectorySeparatorChar))) {
    throw "Refusing to delete outside repo: $resolved"
  }

  Remove-Item -LiteralPath $resolved -Recurse -Force
}

if ($Clean) {
  Write-Host "Cleaning generated build outputs..."
  Remove-GeneratedPath $targetDir
  Remove-GeneratedPath $frontendDist
}

Write-Host "Building Tauri MSI..."
Push-Location $repoRoot
try {
  Write-Host "Running Rust workspace tests..."
  cargo test --workspace

  Write-Host "Building desktop bundle..."
  Write-Host "Tauri beforeBuildCommand will build frontend assets."
  npm run tauri -- build --bundles msi
}
finally {
  Pop-Location
}

$msi = Get-ChildItem -LiteralPath $bundleDir -Filter "*.msi" -File |
  Sort-Object LastWriteTime -Descending |
  Select-Object -First 1
$exePath = Join-Path $releaseDir "$cargoPackageName.exe"
$exe = if (Test-Path -LiteralPath $exePath) {
  Get-Item -LiteralPath $exePath
}
else {
  $null
}

if (-not $msi) {
  throw "MSI was not found under: $bundleDir"
}
if (-not $exe) {
  throw "Executable was not found under: $releaseDir"
}

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$outputDir = Join-Path $OutputRoot "$AppName-$Version-$stamp"
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

Copy-Item -LiteralPath $msi.FullName -Destination $outputDir -Force
Copy-Item -LiteralPath $exe.FullName -Destination (Join-Path $outputDir "$AppName.exe") -Force

@"
$AppName $Version

Build output copied from:
$repoRoot

Build command:
cargo test --workspace
npm run tauri -- build --bundles msi

Frontend build:
Tauri beforeBuildCommand

Clean build:
$($Clean.IsPresent)

Files:
- $($msi.Name): Windows installer.
- $AppName.exe: Release executable from the Tauri build output.
"@ | Set-Content -LiteralPath (Join-Path $outputDir "BUILD-INFO.txt") -Encoding UTF8

Write-Host "Packaged output:"
Get-ChildItem -LiteralPath $outputDir | Select-Object Name, @{Name = "SizeMB"; Expression = { [math]::Round($_.Length / 1MB, 2) } }
Write-Host "Output directory: $outputDir"
