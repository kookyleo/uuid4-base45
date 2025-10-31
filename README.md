# uuid45

[![CI](https://github.com/your-org/uuid45/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/uuid45/actions/workflows/ci.yml)

Compact Base45 codec for UUID v4 by stripping the 6 fixed bits (version+variant), with CLI, Rust library, and WASM bindings.

Features:
- Convert UUID v4 (128-bit) to compact Base45 string by removing the 6 fixed bits (version=4 nibble and RFC4122 variant 2 MSBs), leaving 122 bits. These are packed into 16 bytes (with 6 zero padding bits) and encoded using Base45.
- Decode the Base45 string back to the exact original UUID v4 bytes / string.
- Library API, CLI binary, and WASM bindings for JS. Includes a simple HTML demo.

## Install

- Build CLI: `cargo install --path .`
- Use as a lib: add `uuid45 = { git = "<your repo>" }` or local path.

## CLI usage

```
uuid45

Commands:
  gen                       Generate a random UUID v4 and print Base45 and UUID
  encode <UUID|HEX|@->     Encode a UUID into Base45. Accepts:
                           - canonical UUID string (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
                           - 32-hex (no dashes)
                           - raw 16-byte via stdin with @-
  decode <BASE45|@->       Decode Base45 string back to UUID string and bytes (hex)

Options:
  -q, --quiet              Only print the primary output
  -h, --help               Show this help
```

Examples:
- `uuid45 gen`
- `uuid45 encode 550e8400-e29b-41d4-a716-446655440000`
- `uuid45 decode <base45>`

## Library API

- `generate_v4() -> Uuid`
- `encode_uuid(uuid: Uuid) -> String`
- `encode_uuid_str(s: &str) -> Result<String, Uuid45Error>`
- `encode_uuid_bytes(bytes: &[u8; 16]) -> String`
- `decode_to_uuid(s: &str) -> Result<Uuid, Uuid45Error>`
- `decode_to_bytes(s: &str) -> Result<[u8; 16], Uuid45Error>`
- `decode_to_string(s: &str) -> Result<String, Uuid45Error>`

## WASM usage

This crate exposes the following bindings when compiled for `wasm32-unknown-unknown` with `wasm-bindgen`:
- `wasm_gen_v4() -> String`
- `wasm_encode_uuid_str(s: &str) -> Result<String, JsValue>`
- `wasm_decode_to_uuid_str(s: &str) -> Result<String, JsValue>`
- `wasm_decode_to_bytes(s: &str) -> Result<Uint8Array, JsValue>`

Example site at `examples/wasm/index.html`.

### Build WASM locally (wasm-pack alternative included)

- Install wasm target and wasm-bindgen-cli:

```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript \
  --out-dir examples/wasm/pkg \
  --out-name uuid45 \
  target/wasm32-unknown-unknown/release/uuid45.wasm

# Or using wasm-pack
# cargo install wasm-pack
# wasm-pack build --target web --out-dir examples/wasm/pkg --out-name uuid45
```

- Open `examples/wasm/index.html` via a static server (e.g., `python3 -m http.server`) and navigate to it.

## Tests

`cargo test` includes:
- Random UUID roundtrips
- Known UUID roundtrip
- Padding bits verification

## Why 122 bits?

UUID v4 reserves 4 bits for version and 2 bits for variant. These are fixed values (0100 and 10). Removing them yields 128-6 = 122 bits of entropy. We pack these into 16 bytes, with only 2 bits used in the last byte; the upper 6 bits of the last byte are zero padding. Base45 then encodes these bytes.

## License

MIT
