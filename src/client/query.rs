//! Query attestations with filtering.

use crate::error::EasError;
use crate::types::Attestation;

use super::EasClient;

/// Filter criteria for querying attestations.
#[derive(Debug, Clone, Default)]
pub struct AttestationFilter {
    /// Filter by schema UID.
    pub schema_uid: Option<String>,

    /// Filter by attester address.
    pub attester: Option<String>,

    /// Filter by recipient address.
    pub recipient: Option<String>,

    /// Only include non-revoked attestations.
    pub exclude_revoked: bool,
}

impl EasClient {
    /// Query a single attestation by UID, with optional revocation check.
    ///
    /// This wraps `get_attestation` with an additional revocation filter.
    pub async fn query_attestation(
        &self,
        uid: &str,
        exclude_revoked: bool,
    ) -> Result<Option<Attestation>, EasError> {
        let att = self.get_attestation(uid).await?;

        if exclude_revoked && att.revoked {
            return Ok(None);
        }

        Ok(Some(att))
    }

    /// Check whether an attestation exists and is valid (not revoked, not expired).
    pub async fn is_valid(&self, uid: &str) -> Result<bool, EasError> {
        let att = self.get_attestation(uid).await?;

        if att.revoked {
            return Ok(false);
        }

        if att.expiration_time > 0 {
            let now = chrono::Utc::now().timestamp() as u64;
            if now > att.expiration_time {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Fetch multiple attestations by UIDs.
    ///
    /// Returns results in the same order as the input UIDs.
    /// Skips UIDs that fail to resolve (logs warning but doesn't error).
    pub async fn get_attestations(
        &self,
        uids: &[String],
    ) -> Result<Vec<Attestation>, EasError> {
        let mut results = Vec::with_capacity(uids.len());

        for uid in uids {
            match self.get_attestation(uid).await {
                Ok(att) => results.push(att),
                Err(e) => {
                    tracing::warn!(uid = %uid, error = %e, "failed to fetch attestation, skipping");
                }
            }
        }

        Ok(results)
    }
}
