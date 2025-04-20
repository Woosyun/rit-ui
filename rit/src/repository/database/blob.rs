use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Self (content)
    }
    pub fn content(&self) -> &str {
        &self.0
    }
}
