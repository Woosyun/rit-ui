use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Oid([u8; 32]);
impl Oid {
    pub fn build(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize()
            .into();

        Self(result) 
    }

    pub fn split(&self) -> (String, String) {
        let mut hex = self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>();
        let file = hex.split_off(2).join("");
        let dir = hex.join("");

        (dir, file)
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex_oid = self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", hex_oid)
    }
}
