//! Core data types — implemented in Step 3

/// On-chain attestation record.
#[derive(Debug, Clone)]
pub struct Attestation {
    pub uid: String,
}

/// Request to create a new attestation.
#[derive(Debug, Clone)]
pub struct AttestationRequest {
    pub schema_uid: String,
}

/// EAS schema record.
#[derive(Debug, Clone)]
pub struct SchemaRecord {
    pub uid: String,
}
