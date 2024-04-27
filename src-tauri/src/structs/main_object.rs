use serde::{Deserialize, Serialize};
use crate::structs::{RequestObject, ResponseObject};

#[derive(Serialize, Deserialize, Debug)]
pub struct MainObject {
    pub http_version: String,
    pub request: RequestObject,
    pub response: ResponseObject
}

pub type MainObjects = Vec<MainObject>;