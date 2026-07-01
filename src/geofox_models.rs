use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCRequest {
    pub(crate) version: u8,
    pub(crate) postal_code: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCResponse {
    pub(crate) return_code: String,
    #[serde(rename = "isHVV")]
    pub(crate) is_hvv: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LSRequest {
    pub(crate) data_release_id: String,
    pub(crate) modification_types: Vec<String>,
    pub(crate) coordinate_type: String,
    pub(crate) filter_equivalent: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LSResponse {
    pub(crate) return_code: String,
    #[serde(rename = "dataReleaseID")]
    pub(crate) data_release_id: String,
    pub(crate) stations: Vec<StationListEntry>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationListEntry {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) city: Option<String>,
    pub(crate) combined_name: Option<String>,
    pub(crate) shortcuts: Option<Vec<String>>,
    pub(crate) aliasses: Option<Vec<String>>,
    pub(crate) vehicle_types: Option<Vec<String>>,
    pub(crate) coordinate: Option<Coordinate>,
    pub(crate) exists: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNRequest {
    pub(crate) the_name: String,
    pub(crate) max_list_l: u16,
    pub(crate) max_distance: u16,
    pub(crate) coordinate_type: String,
    pub(crate) tariff_details: bool,
    pub(crate) allow_type_switch: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    x: u64,
    y: u64,
}
