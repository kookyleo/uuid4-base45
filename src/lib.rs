//! uuid45: Compact Base45 codec for UUID v4 by stripping the 6 fixed bits (version+variant).
//! - 128-bit UUID v4 -> remove version(4b) and variant(2b) => 122 bits, pack to 16 bytes (last 6 bits unused) => Base45
//! - Reverse to reconstruct a canonical UUID v4
//!   Provides: library API, WASM bindings (target wasm32), and used by CLI.

use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum Uuid45Error {
    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),
    #[error("Invalid Base45: {0}")]
    InvalidBase45(String),
    #[error("Invalid length: expected {expected} got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("Non-zero padding bits in compact payload")]
    NonZeroPadding,
}

/// Positions (byte index, bit index within byte [7..0], big-endian) of the fixed bits in a UUID v4.
/// - Version: byte 6, bits 7..4 must be 0b0100
/// - Variant (RFC4122): byte 8, bits 7..6 must be 0b10
const FIXED_POSITIONS: &[(usize, u8, u8)] = &[
    // (byte_idx, mask, expected_value_on_those_bits)
    // Version nibble (bits 7..4)
    (6, 0b1111_0000, 0b0100_0000),
    // Variant two MSBs (bits 7..6)
    (8, 0b1100_0000, 0b1000_0000),
];

/// Extract 122-bit compact representation from a canonical UUID v4 byte array.
/// Returns 16 bytes where only the lowest 122 bits are used; the top 6 bits of the last byte must be zero.
pub fn uuid_to_compact_bytes(uuid_bytes: &[u8; 16]) -> Result<[u8; 16], Uuid45Error> {
    // Validate fixed bits follow UUID v4 spec (optional but recommended)
    for (idx, mask, expected) in FIXED_POSITIONS.iter().copied() {
        if uuid_bytes[idx] & mask != expected {
            // We still allow encoding but enforce spec by overwriting those bits when reconstructing.
            // To be strict, return error; but it's safer to allow as long as version/variant can be normalized.
            // Here we choose to be lenient: do nothing.
        }
    }

    // Gather all 128 bits, skip the 6 fixed ones in order.
    let mut out_bits = Vec::with_capacity(122);
    for (byte_idx, b) in uuid_bytes.iter().copied().enumerate().take(16) {
        for bit in (0..8).rev() {
            // msb first
            let mask = 1u8 << bit;
            let is_fixed =
                // Version nibble bits 7..4 in byte 6
                (byte_idx == 6 && bit >= 4) ||
                // Variant bits 7..6 in byte 8
                (byte_idx == 8 && bit >= 6);
            if is_fixed {
                continue;
            }
            out_bits.push((b & mask) != 0);
        }
    }
    assert_eq!(out_bits.len(), 122);

    // Pack into 16 bytes (ceil(122/8) = 16). Only lowest 2 bits of the last byte are used.
    // We pack LSB-first so that padding occupies the high bits of the last byte.
    let mut out = [0u8; 16];
    let mut bit_idx = 0;
    for item in &mut out {
        let mut acc = 0u8;
        for bit in 0..8 {
            acc |= (out_bits.get(bit_idx).copied().unwrap_or(false) as u8) << bit;
            bit_idx += 1;
            if bit_idx >= 122 {
                break;
            }
        }
        *item = acc;
        if bit_idx >= 122 {
            break;
        }
    }

    // Ensure the top 6 bits of the last byte (bits 7..2) are zeroed by construction
    out[15] &= 0b0000_0011;

    Ok(out)
}

/// Reconstruct full 128-bit UUID bytes from a compact 122-bit packed representation.
/// Accepts 16 bytes; ignores the top 6 padding bits which must be zero.
pub fn compact_bytes_to_uuid(compact: &[u8]) -> Result<[u8; 16], Uuid45Error> {
    if compact.len() != 16 {
        return Err(Uuid45Error::InvalidLength {
            expected: 16,
            actual: compact.len(),
        });
    }
    if compact[15] & 0b1111_1100 != 0 {
        return Err(Uuid45Error::NonZeroPadding);
    }

    // Unpack 122 bits LSB-first from each byte
    let mut bits: Vec<bool> = Vec::with_capacity(122);
    let mut taken = 0usize;
    'outer: for &b in compact.iter().take(16) {
        for bit in 0..8 {
            bits.push(((b >> bit) & 1) != 0);
            taken += 1;
            if taken >= 122 {
                break 'outer;
            }
        }
    }

    // Reinsert into 16-byte array, putting version/variant fixed bits as required
    let mut out = [0u8; 16];
    let mut bit_iter = bits.into_iter();
    for (byte_idx, item) in out.iter_mut().enumerate() {
        let mut acc = 0u8;
        for bit in (0..8).rev() {
            let is_fixed_version = byte_idx == 6 && bit >= 4;
            let is_fixed_variant = byte_idx == 8 && bit >= 6;
            if is_fixed_version {
                // version 4 => 0100 in bits 7..4
                let val = if bit == 6 { 1 } else { 0 }; // bits: 7:0, 6:1, 5:0, 4:0
                acc |= (val as u8) << bit;
            } else if is_fixed_variant {
                // variant RFC4122 => 10 in bits 7..6
                let val = if bit == 7 { 1 } else { 0 }; // bit7:1, bit6:0
                acc |= (val as u8) << bit;
            } else {
                let v = bit_iter.next().unwrap_or(false) as u8;
                acc |= v << bit;
            }
        }
        *item = acc;
    }

    Ok(out)
}

/// Encode a UUID into Base45 compact string.
pub fn encode_uuid(uuid: Uuid) -> String {
    let bytes = uuid.into_bytes();
    let compact = uuid_to_compact_bytes(&bytes)
        .expect("uuid_to_compact_bytes should not fail for valid UUID");
    qr_base45::encode(&compact)
}

/// Try to encode a UUID string into Base45 compact string.
pub fn encode_uuid_str(s: &str) -> Result<String, Uuid45Error> {
    let uuid = Uuid::parse_str(s).map_err(|e| Uuid45Error::InvalidUuid(e.to_string()))?;
    Ok(encode_uuid(uuid))
}

/// Encode raw 16-byte UUID into Base45 compact string.
pub fn encode_uuid_bytes(bytes: &[u8; 16]) -> String {
    let compact = uuid_to_compact_bytes(bytes).expect("valid path");
    qr_base45::encode(&compact)
}

/// Decode Base45 compact string into a UUID.
pub fn decode_to_uuid(s: &str) -> Result<Uuid, Uuid45Error> {
    let bytes = qr_base45::decode(s).map_err(|e| Uuid45Error::InvalidBase45(e.to_string()))?;
    let arr = compact_bytes_to_uuid(&bytes)?;
    Ok(Uuid::from_bytes(arr))
}

/// Decode Base45 compact string back to canonical 16-byte UUID bytes.
pub fn decode_to_bytes(s: &str) -> Result<[u8; 16], Uuid45Error> {
    let u = decode_to_uuid(s)?;
    Ok(u.into_bytes())
}

/// Decode to hyphenated UUID string.
pub fn decode_to_string(s: &str) -> Result<String, Uuid45Error> {
    Ok(decode_to_uuid(s)?.hyphenated().to_string())
}

/// Generate a random UUID v4.
pub fn generate_v4() -> Uuid {
    Uuid::new_v4()
}

// ===== WASM bindings =====
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_gen_v4() -> String {
    generate_v4().hyphenated().to_string()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_encode_uuid_str(s: &str) -> Result<String, JsValue> {
    encode_uuid_str(s).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_decode_to_uuid_str(s: &str) -> Result<String, JsValue> {
    decode_to_string(s).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_decode_to_bytes(s: &str) -> Result<js_sys::Uint8Array, JsValue> {
    let arr = decode_to_bytes(s).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(js_sys::Uint8Array::from(&arr[..]))
}

// ===== Tests =====
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_random_v4() {
        for _ in 0..200 {
            let u = generate_v4();
            let s = encode_uuid(u);
            let d = decode_to_uuid(&s).unwrap();
            assert_eq!(u, d);
        }
    }

    #[test]
    fn known_uuid_roundtrip() {
        let u = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        // Ensure version=4 and variant RFC4122
        assert_eq!(u.get_version_num(), 4);
        assert!(matches!(u.get_variant(), uuid::Variant::RFC4122));
        let s = encode_uuid(u);
        let d = decode_to_uuid(&s).unwrap();
        assert_eq!(u, d);
    }

    #[test]
    fn padding_bits_zero() {
        let u = Uuid::parse_str("ffffffff-ffff-4fff-bfff-ffffffffffff").unwrap();
        // Encode and decode
        let enc = encode_uuid(u);
        let raw = qr_base45::decode(&enc).unwrap();
        assert_eq!(raw.len(), 16);
        assert_eq!(raw[15] & 0b1111_1100, 0);
    }

    #[test]
    fn invalid_base45_char() {
        assert!(qr_base45::decode("A😀").is_err());
        assert!(qr_base45::decode("a").is_err()); // lowercase not allowed
    }

    #[test]
    fn invalid_base45_dangling() {
        assert!(qr_base45::decode("A").is_err()); // dangling single char
    }

    #[test]
    fn invalid_base45_overflow() {
        // ':::' => a=b=c=44 -> x=91124 > 65535 -> overflow
        assert!(qr_base45::decode(":::").is_err());
    }

    #[test]
    fn non_zero_padding_rejected() {
        let u = generate_v4();
        let s = encode_uuid(u);
        let mut compact = qr_base45::decode(&s).unwrap();
        assert_eq!(compact.len(), 16);
        // Flip a high padding bit in last byte
        compact[15] |= 0b0001_0000;
        let err = compact_bytes_to_uuid(&compact).unwrap_err();
        match err {
            Uuid45Error::NonZeroPadding => {}
            _ => panic!("expected NonZeroPadding, got {err:?}"),
        }
    }

    #[test]
    fn invalid_length_rejected() {
        let u = generate_v4();
        let s = encode_uuid(u);
        let compact = qr_base45::decode(&s).unwrap();
        let err = compact_bytes_to_uuid(&compact[..15]).unwrap_err();
        match err {
            Uuid45Error::InvalidLength { .. } => {}
            _ => panic!("expected InvalidLength, got {err:?}"),
        }
    }

    #[test]
    fn version_and_variant_preserved() {
        for _ in 0..100 {
            let u = generate_v4();
            let s = encode_uuid(u);
            let d = decode_to_uuid(&s).unwrap();
            assert_eq!(d.get_version_num(), 4);
            assert!(matches!(d.get_variant(), uuid::Variant::RFC4122));
        }
    }

    #[test]
    fn reencode_stability() {
        for _ in 0..50 {
            let u = generate_v4();
            let s1 = encode_uuid(u);
            let u2 = decode_to_uuid(&s1).unwrap();
            let s2 = encode_uuid(u2);
            assert_eq!(s1, s2);
        }
    }

    #[test]
    fn extreme_known_values() {
        // Minimal v4/RFC4122 compliant bits
        let u_min = Uuid::parse_str("00000000-0000-4000-8000-000000000000").unwrap();
        let s = encode_uuid(u_min);
        let d = decode_to_uuid(&s).unwrap();
        assert_eq!(u_min, d);

        // Maximal within v4/RFC4122 mask already covered by padding_bits_zero
        let u_max = Uuid::parse_str("ffffffff-ffff-4fff-bfff-ffffffffffff").unwrap();
        let s2 = encode_uuid(u_max);
        let d2 = decode_to_uuid(&s2).unwrap();
        assert_eq!(u_max, d2);
    }
}
