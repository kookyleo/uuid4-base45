#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
EX_DIR="$ROOT_DIR/examples/wasm"
PKG_DIR="$EX_DIR/pkg"
PORT=${PORT:-10080}

echo "[1/4] Ensuring wasm32 target..."
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

mkdir -p "$PKG_DIR"
mkdir -p "$EX_DIR/vendor"
if command -v wasm-pack >/dev/null 2>&1; then
  echo "[2/4] Building with wasm-pack..."
  (cd "$ROOT_DIR" && wasm-pack build --target web --out-dir "$PKG_DIR" --out-name uuid45)
else
  echo "[2/4] wasm-pack not found; using wasm-bindgen"
  if ! command -v wasm-bindgen >/dev/null 2>&1; then
    echo "wasm-bindgen not found. Install with: cargo install wasm-bindgen-cli" >&2
    exit 1
  fi
  echo "[2/4] Building with cargo (release wasm32)..."
  (cd "$ROOT_DIR" && cargo build --release --target wasm32-unknown-unknown)
  echo "[3/4] Generating JS bindings..."
  wasm-bindgen --target web --no-typescript \
    --out-dir "$PKG_DIR" \
    --out-name uuid45 \
    "$ROOT_DIR/target/wasm32-unknown-unknown/release/uuid45.wasm"
fi

cd "$EX_DIR"
echo "[4/4] Serving $EX_DIR at http://localhost:$PORT ..."
# Prefer python3 http.server
if command -v python3 >/dev/null 2>&1; then
  python3 -m http.server "$PORT"
elif command -v python >/dev/null 2>&1; then
  python -m SimpleHTTPServer "$PORT"
else
  echo "No python found. Please serve this directory with any static server." >&2
  exit 1
fi
