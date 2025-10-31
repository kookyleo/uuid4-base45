#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$RootDir = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$ExDir = Join-Path $RootDir 'examples/wasm'
$PkgDir = Join-Path $ExDir 'pkg'
$Port = $env:PORT
if (-not $Port) { $Port = 8080 }

Write-Host "[1/4] Ensuring wasm32 target..."
try { rustup target add wasm32-unknown-unknown | Out-Null } catch {}

New-Item -ItemType Directory -Force -Path $PkgDir | Out-Null
New-Item -ItemType Directory -Force -Path (Join-Path $ExDir 'vendor') | Out-Null
# fetch vendor QR lib if missing
$VendorPath = Join-Path $ExDir 'vendor/qrcode.js'
if (-not (Test-Path $VendorPath)) {
  Write-Host "[2/4] Fetching QR vendor lib..."
  try { Invoke-WebRequest -Uri https://unpkg.com/qrcode-generator@1.4.4/qrcode.js -OutFile $VendorPath -UseBasicParsing }
  catch { Invoke-WebRequest -Uri https://cdn.jsdelivr.net/npm/qrcode-generator@1.4.4/qrcode.js -OutFile $VendorPath -UseBasicParsing }
}

if (Get-Command wasm-pack -ErrorAction SilentlyContinue) {
  Write-Host "[2/4] Building with wasm-pack..."
  Push-Location $RootDir
  wasm-pack build --target web --out-dir $PkgDir --out-name uuid45
  Pop-Location
}
else {
  Write-Host "[2/4] wasm-pack not found; using wasm-bindgen"
  if (-not (Get-Command wasm-bindgen -ErrorAction SilentlyContinue)) {
    Write-Error "wasm-bindgen not found. Install with: cargo install wasm-bindgen-cli"
    exit 1
  }
  Write-Host "[2/4] Building with cargo (release wasm32)..."
  Push-Location $RootDir
  cargo build --release --target wasm32-unknown-unknown
  Pop-Location
  Write-Host "[3/4] Generating JS bindings..."
  $WasmPath = Join-Path $RootDir 'target/wasm32-unknown-unknown/release/uuid45.wasm'
  wasm-bindgen --target web --no-typescript `
    --out-dir $PkgDir `
    --out-name uuid45 `
    $WasmPath
}

Set-Location $ExDir
Write-Host "[4/4] Serving $ExDir at http://localhost:$Port ..."
if (Get-Command python3 -ErrorAction SilentlyContinue) {
  python3 -m http.server $Port
} elseif (Get-Command python -ErrorAction SilentlyContinue) {
  python -m SimpleHTTPServer $Port
} else {
  Write-Error "No python found. Please serve this directory with any static server."
  exit 1
}
