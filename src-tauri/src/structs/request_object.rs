use serde::{Deserialize, Serialize};

use super::HeaderObject;
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestObject {
    pub body: Option<String>,
    pub headers: Vec<HeaderObject>,
    pub method: String,
    pub url: String
}