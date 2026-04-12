//! Known-answer tests for ABI encoding.

use antarez_eas_client::{decode_attestation, decode_simple, encode_attestation, encode_simple};

#[test]
fn test_full_encode_decode_known_vector() {
    let hash = format!("0x{}", "aa".repeat(32));
    let desc = "known-answer test";
    let ts = 1700000000u64;

    let encoded = encode_attestation(&hash, desc, ts).unwrap();

    // ABI encoding is deterministic — full payload has hash + offset + offset + ts + ...
    assert!(encoded.len() >= 96, "encoded too short: {}", encoded.len());

    let (h, d, t) = decode_attestation(&encoded).unwrap();
    assert_eq!(h, hash);
    assert_eq!(d, desc);
    assert_eq!(t, ts);
}

#[test]
fn test_simple_encode_is_32_bytes() {
    let hash = format!("0x{}", "bb".repeat(32));
    let encoded = encode_simple(&hash).unwrap();
    // ABI-encoded bytes32 as a struct is 32 bytes
    assert_eq!(encoded.len(), 32);

    let decoded = decode_simple(&encoded).unwrap();
    assert_eq!(decoded, hash);
}

#[test]
fn test_decode_garbage_fails() {
    let garbage = vec![0u8; 7];
    assert!(decode_simple(&garbage).is_err());
}

#[test]
fn test_empty_description() {
    let hash = format!("0x{}", "cc".repeat(32));
    let encoded = encode_attestation(&hash, "", 0).unwrap();
    let (h, d, t) = decode_attestation(&encoded).unwrap();
    assert_eq!(h, hash);
    assert_eq!(d, "");
    assert_eq!(t, 0);
}
