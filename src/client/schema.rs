//! Schema registration operations on the EAS Schema Registry.

use alloy::primitives::{Address, FixedBytes};
use alloy::sol;

use crate::error::{ContractError, EasError};
use crate::types::{SchemaRecord, SchemaRequest};

use super::EasClient;

// Generate type-safe bindings for the Schema Registry contract.
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    interface ISchemaRegistry {
        struct SchemaRegistryRecord {
            bytes32 uid;
            address resolver;
            bool revocable;
            string schema;
        }

        function register(string calldata schema, address resolver, bool revocable) external returns (bytes32);
        function getSchema(bytes32 uid) external view returns (SchemaRegistryRecord memory);
    }
}

impl EasClient {
    /// Register a new schema on the Schema Registry.
    ///
    /// Returns the schema record with its assigned UID.
    pub async fn register_schema(
        &self,
        request: &SchemaRequest,
    ) -> Result<SchemaRecord, EasError> {
        let resolver: Address = request
            .resolver
            .parse()
            .map_err(|e: alloy::hex::FromHexError| EasError::Config {
                message: format!("invalid resolver address: {e}"),
            })?;

        let registry = ISchemaRegistry::new(self.schema_registry_address, &*self.provider);

        let call = registry.register(
            request.schema.clone(),
            resolver,
            request.revocable,
        );

        let pending = call
            .send()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        let receipt = pending
            .with_required_confirmations(self.confirmations)
            .get_receipt()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::TransactionFailed {
                    details: e.to_string(),
                })
            })?;

        // Extract schema UID from the Registered event log.
        let uid = extract_schema_uid(&receipt)?;

        Ok(SchemaRecord {
            uid: format!("0x{}", hex::encode(uid)),
            schema: request.schema.clone(),
            resolver: request.resolver.clone(),
            revocable: request.revocable,
        })
    }

    /// Get an existing schema record by UID.
    pub async fn get_schema(&self, uid: &str) -> Result<SchemaRecord, EasError> {
        let uid_bytes = parse_schema_bytes32(uid)?;
        let registry = ISchemaRegistry::new(self.schema_registry_address, &*self.provider);

        let result = registry
            .getSchema(uid_bytes)
            .call()
            .await
            .map_err(|e| {
                EasError::Contract(ContractError::CallFailed {
                    details: e.to_string(),
                })
            })?;

        Ok(SchemaRecord {
            uid: format!("0x{}", hex::encode(result.uid)),
            schema: result.schema,
            resolver: format!("{:?}", result.resolver),
            revocable: result.revocable,
        })
    }
}

// --- Helpers ---

fn parse_schema_bytes32(hex_str: &str) -> Result<FixedBytes<32>, EasError> {
    let stripped = hex_str.strip_prefix("0x").unwrap_or(hex_str);
    let bytes = hex::decode(stripped).map_err(|e| {
        EasError::Contract(ContractError::CallFailed {
            details: format!("bad hex: {e}"),
        })
    })?;
    let arr: [u8; 32] = bytes.try_into().map_err(|_| {
        EasError::Contract(ContractError::CallFailed {
            details: "expected 32 bytes".into(),
        })
    })?;
    Ok(FixedBytes(arr))
}

/// Extract schema UID from the Registered event in the transaction receipt.
fn extract_schema_uid(
    receipt: &alloy::rpc::types::TransactionReceipt,
) -> Result<[u8; 32], EasError> {
    for log in receipt.inner.logs() {
        if !log.data().data.is_empty() && log.data().data.len() >= 32 {
            let uid: [u8; 32] = log.data().data[..32].try_into().map_err(|_| {
                EasError::Contract(ContractError::CallFailed {
                    details: "bad log data".into(),
                })
            })?;
            return Ok(uid);
        }
    }
    Err(EasError::Contract(ContractError::CallFailed {
        details: "no Registered event found".into(),
    }))
}
