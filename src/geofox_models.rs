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
    #[serde(rename = "dataReleaseID")]
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
    pub(crate) stations: Option<Vec<StationListEntry>>,
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
    pub(crate) the_name: SDName,
    pub(crate) max_list_l: u16,
    pub(crate) max_distance: u16,
    pub(crate) coordinate_type: String,
    pub(crate) tariff_details: bool,
    pub(crate) allow_type_switch: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    x: f32,
    y: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SDName {
    pub(crate) name: Option<String>,
    pub(crate) city: Option<String>,
    pub(crate) combined_name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) sd_type: Option<String>,
    pub(crate) coordinate: Option<Coordinate>,
    pub(crate) layer: Option<i16>,
    pub(crate) tariff_details: Option<TariffDetail>,
    pub(crate) has_station_information: Option<bool>,
    pub(crate) provider: Option<String>,
    pub(crate) address: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TariffDetail {
    pub(crate) inner_city: Option<String>,
    pub(crate) city_traffic: Option<String>,
    pub(crate) gratis: Option<bool>,
    pub(crate) greater_area: Option<bool>,
    pub(crate) sh_village_id: Option<i16>,
    pub(crate) sh_tariff_zones: Option<Vec<i16>>,
    pub(crate) tariff_zones: Option<Vec<i16>>,
    pub(crate) counties: Option<Vec<String>>,
    pub(crate) rings: Option<Vec<String>>,
    pub(crate) fare_stage: Option<bool>,
    pub(crate) fare_stage_number: Option<i16>,
    pub(crate) tariff_names: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalSDName {
    pub(crate) name: Option<String>,
    pub(crate) city: Option<String>,
    pub(crate) combined_name: Option<String>,
    #[serde(rename = "type")]
    pub(crate) sd_type: Option<String>,
    pub(crate) coordinate: Option<Coordinate>,
    pub(crate) layer: Option<i16>,
    pub(crate) tariff_details: Option<TariffDetail>,
    pub(crate) has_station_information: Option<bool>,
    pub(crate) provider: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) distance: Option<i32>,
    pub(crate) time: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNResponse {
    pub(crate) return_code: String,
    pub(crate) results: Option<Vec<RegionalSDName>>
}