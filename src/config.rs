//! Client configuration — implemented in Step 3

/// EAS client configuration.
#[derive(Debug, Clone)]
pub struct EasConfig {
    pub rpc_url: String,
    pub eas_contract_address: String,
    pub schema_registry_address: String,
}
