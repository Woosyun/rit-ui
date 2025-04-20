use serde::{Serialize, Deserialize};
use serde_json;

pub fn decode<'a, O: Serialize>(o: &O) -> Result<String, String> {
    serde_json::to_string(o)
        .map_err(|e| e.to_string())
}

pub fn encode<'a, O: Deserialize<'a>>(content: &'a str) -> Result<O, String> {
    serde_json::from_str(content)
        .map_err(|e| e.to_string())
}
