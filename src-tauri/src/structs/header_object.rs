use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct HeaderObject {
    pub name: String,
    pub value: String
}