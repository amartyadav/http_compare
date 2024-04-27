use serde::{Deserialize, Serialize};

use super::HeaderObject;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseObject {
    pub body: Option<String>,
    pub headers: Vec<HeaderObject>,
    pub status_code: u16,
    pub status_text: String
}