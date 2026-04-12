//! Basic usage example for antarez-eas-client.
//!
//! Run: cargo run --example basic_usage
//! Requires: RPC_URL and PRIVATE_KEY environment variables.

use antarez_eas_client::{
    EasClient, EasConfig, AttestationRequest, SchemaRequest,
    chain::SEPOLIA,
    encode_simple,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
    let private_key = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY env var required");

    println!("Connecting to {rpc_url}...");

    // Use Sepolia testnet config.
    let config = EasConfig::for_chain(&SEPOLIA, &rpc_url);
    let client = EasClient::new(&config, &private_key).await?;

    println!("Connected to chain {}", client.chain_id());

    // Register a schema.
    let schema_request = SchemaRequest {
        schema: "bytes32 dataHash".to_string(),
        resolver: "0x0000000000000000000000000000000000000000".to_string(),
        revocable: false,
    };

    let schema = client.register_schema(&schema_request).await?;
    println!("Schema registered: {}", schema.uid);

    // Create an attestation.
    let data_hash = format!("0x{}", "ab".repeat(32));
    let encoded = encode_simple(&data_hash)?;

    let att_request = AttestationRequest::simple(&schema.uid, encoded);
    let attestation = client.create_attestation(&att_request).await?;
    println!("Attestation created: {}", attestation.uid);

    // Verify it exists.
    let fetched = client.get_attestation(&attestation.uid).await?;
    println!("Attestation verified: uid={}", fetched.uid);

    // Check validity.
    let valid = client.is_valid(&attestation.uid).await?;
    println!("Attestation valid: {valid}");

    Ok(())
}
