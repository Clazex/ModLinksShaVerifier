use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
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
    pub fn verify(&self) -> (bool, String) {
        match self.verify_impl() {
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

    fn verify_impl(&self) -> Result<String> {
        let client = Client::builder().build().map_err(|e| {
            anyhow!("Failed to initialize connection\n::error title=Connection Error::{e}")
        })?;

        let res = client
            .get(&self.url)
            .send()
            .map_err(|e| anyhow!("Failed to download\n::error title=Connection Error::{e}"))?;

        let bytes = res
            .bytes()
            .map_err(|e| anyhow!("Failed to read response\n::error title=Connection Error::{e}"))?;

        let raw_hash = Sha256::digest(bytes);
        Ok(base16ct::upper::encode_string(&raw_hash))
    }
}
