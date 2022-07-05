use serde::Deserialize;

use super::{Links, Verify};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ApiLinks {
    #[serde(rename = "Manifest")]
    manifest: Api,
}

impl Verify for ApiLinks {
    fn verify(&self, client: &ureq::Agent) -> bool {
        self.manifest.verify(client)
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct Api {
    #[serde(flatten)]
    links: Links,
}

impl Api {
    pub fn verify(&self, client: &ureq::Agent) -> bool {
        let (res, msg) = self.links.verify(client);

        println!(
            "API |{}| {msg}",
            match res {
                true => '✅',
                false => '❌',
            }
        );

        res
    }
}
