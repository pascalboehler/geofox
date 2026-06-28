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

#[derive(Serialize, Deserialize)]
pub struct LSRequest {
    pub(crate) dataReleaseID: String,
    pub(crate) modificationTypes: Vec<String>,
    pub(crate) coordinateType: String,
    pub(crate) filterEquivalent: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LSResponse {
    pub(crate) dataReleaseID: String,
    pub(crate) stations: Vec<StationListEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct StationListEntry {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) city: String,
    pub(crate) combinedName: String,
    pub(crate) shortcuts: Vec<String>,
    pub(crate) aliasses: Vec<String>,
    pub(crate) vehicleTypes: Vec<String>,
    pub(crate) coordinate: Coordinate,
    pub(crate) exists: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CNRequest {
    pub(crate) theName: String,
    pub(crate) maxListL: u16,
    pub(crate) maxDistance: u16,
    pub(crate) coordinateType: String,
    pub(crate) tariffDetails: bool,
    pub(crate) allowTypeSwitch: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    x: u64,
    y: u64,
}
