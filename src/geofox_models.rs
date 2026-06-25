use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PCRequest {
    pub(crate) version: u8,
    pub(crate) postalCode: u16,
}

#[derive(Serialize, Deserialize)]
pub struct PCResponse {
    pub(crate) returnCode: String,
    pub(crate) isHVV: bool,
}
