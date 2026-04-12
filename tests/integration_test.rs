//! Integration tests using Anvil local testnet.
//!
//! These tests require `anvil` to be installed (`foundryup`).
//! Run with: cargo test --test integration_test -- --ignored

use antarez_eas_client::{EasClient, EasConfig};

/// Skip-friendly helper: check if anvil is available.
fn anvil_available() -> bool {
    std::process::Command::new("anvil")
        .arg("--version")
        .output()
        .is_ok()
}

#[tokio::test]
#[ignore = "requires anvil and EAS contract deployment"]
async fn test_client_connects_to_anvil() {
    if !anvil_available() {
        eprintln!("anvil not found, skipping integration test");
        return;
    }

    // Spawn a local Anvil instance.
    let mut anvil = tokio::process::Command::new("anvil")
        .arg("--port")
        .arg("18545")
        .arg("--silent")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("failed to start anvil");

    // Give Anvil time to start.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Anvil default account #0 private key.
    let pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    let config = EasConfig {
        rpc_url: "http://127.0.0.1:18545".to_string(),
        eas_contract_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        schema_registry_address: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".to_string(),
        chain_id: 31337, // Anvil default
        tx_timeout_secs: 30,
        confirmations: 1,
    };

    let client = EasClient::new(&config, pk).await;
    assert!(
        client.is_ok(),
        "client creation should succeed: {:?}",
        client.err()
    );

    let client = client.unwrap();
    assert_eq!(client.chain_id(), 31337);

    anvil.kill().await.ok();
}

#[tokio::test]
#[ignore = "requires anvil and EAS contract deployment"]
async fn test_chain_id_mismatch_detected() {
    if !anvil_available() {
        eprintln!("anvil not found, skipping integration test");
        return;
    }

    let mut anvil = tokio::process::Command::new("anvil")
        .arg("--port")
        .arg("18546")
        .arg("--silent")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("failed to start anvil");

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    // Deliberately set the wrong chain ID.
    let config = EasConfig {
        rpc_url: "http://127.0.0.1:18546".to_string(),
        eas_contract_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3".to_string(),
        schema_registry_address: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512".to_string(),
        chain_id: 1, // Wrong — Anvil is 31337
        tx_timeout_secs: 30,
        confirmations: 1,
    };

    let result = EasClient::new(&config, pk).await;
    assert!(result.is_err(), "should fail with chain ID mismatch");

    let err = match result {
        Err(e) => format!("{e}"),
        Ok(_) => panic!("expected error"),
    };
    assert!(
        err.contains("mismatch"),
        "error should mention mismatch: {err}"
    );

    anvil.kill().await.ok();
}
