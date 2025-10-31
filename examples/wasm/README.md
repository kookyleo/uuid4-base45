# WASM Demo for uuid45

This example demonstrates using the uuid45 WASM bindings in a simple HTML page.

What it shows:
- Generate UUID v4 in the browser (via wasm)
- Encode UUID to compact Base45 string
- Decode Base45 back to UUID string

## Prerequisites
- Rust toolchain (stable)
- wasm-bindgen-cli or wasm-pack
- A static HTTP server (e.g. Python http.server)

## One-liner build & serve
- 将 qrcode-generator 的 qrcode.js 放入本目录的 vendor/ 目录：
  - https://unpkg.com/qrcode-generator@1.4.4/qrcode.js
  - 或 https://cdn.jsdelivr.net/npm/qrcode-generator@1.4.4/qrcode.js
- Bash (Linux/macOS/Git Bash):
  - `./build_and_serve.sh`
- PowerShell (Windows):
  - `./build_and_serve.ps1`

The script will:
1) Add wasm32 target (if missing)
2) Build the crate for wasm32
3) Generate JS bindings into `examples/wasm/pkg`
4) Launch a local static server and open the demo

## Manual steps

Option A: wasm-bindgen
```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript \
  --out-dir examples/wasm/pkg \
  --out-name uuid45 \
  target/wasm32-unknown-unknown/release/uuid45.wasm
```

Option B: wasm-pack
```
# cargo install wasm-pack
wasm-pack build --target web --out-dir examples/wasm/pkg --out-name uuid45
```

Then serve the folder:
```
cd examples/wasm
python3 -m http.server 8080
# Open http://localhost:8080 in your browser
```
