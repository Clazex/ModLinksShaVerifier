use std::io;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FileDef {
    #[serde(rename = "SHA256")]
    sha256: String,
    #[serde(rename = "$value")]
    url: String,
}

impl FileDef {
    pub fn verify(&self, client: &ureq::Agent) -> (bool, String) {
        match self.verify_impl(client) {
            Ok(hash) => {
                let expected = self.sha256.to_uppercase();
                match expected == hash {
					true => (true, format!("Matched: {expected}")),
					false => (
						false,
						format!(
                    		"Mismatched\n::error title=Hash Mismatched::Expected: {expected}, Actual: {hash}"
              			),
					)
				}
            }
            Err(e) => (false, e.to_string()),
        }
    }

    #[inline]
    fn verify_impl(&self, client: &ureq::Agent) -> Result<String> {
        let res = client
            .get(&self.url)
            .call()
            .map_err(|e| anyhow!("Failed to connect\n::error title=Network Error::{e}"))?;

        let mut hasher = Sha256::new();
        io::copy(&mut res.into_reader(), &mut hasher)
            .map_err(|e| anyhow!("Failed to read response\n::error title=Network Error::{e}"))?;

        let raw_hash = hasher.finalize();
        Ok(base16ct::upper::encode_string(&raw_hash))
    }
}
