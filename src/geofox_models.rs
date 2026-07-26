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
pub struct LLRequest {
    #[serde(rename = "dataReleaseID")]
    pub(crate) data_release_id: String,
    pub(crate) modification_types: Vec<String>,
    pub(crate) with_sublines: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LLResponse {
    #[serde(rename = "dataReleaseID")]
    pub(crate) data_release_id: String,
    pub(crate) lines: Option<Vec<LineListEntry>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineListEntry {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) carrier_name_short: Option<String>,
    pub(crate) carrier_long_name: Option<String>,
    pub(crate) sublines: Option<Vec<SublineListEntry>>,
    pub(crate) exists: Option<bool>,
    #[serde(rename = "type")]
    pub(crate) service_type: Option<ServiceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceType {
    pub(crate) simple_type: String,
    pub(crate) short_info: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SublineListEntry {
    pub(crate) subline_number: String,
    pub(crate) vehicle_type: String,
    pub(crate) station_sequence: Vec<StationLight>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationLight {
    pub(crate) id: String,
    pub(crate) name: String,
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

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
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
    pub(crate) tariff_names: Option<Vec<String>>,
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
    pub(crate) time: Option<String>,
}

impl RegionalSDName {
    pub fn to_sd_name(&self) -> SDName {
        SDName {
            name: self.name.clone(),
            city: self.city.clone(),
            combined_name: self.combined_name.clone(),
            sd_type: self.sd_type.clone(),
            coordinate: self.coordinate.clone(),
            layer: self.layer,
            tariff_details: self.tariff_details.clone(),
            has_station_information: self.has_station_information,
            provider: self.provider.clone(),
            address: self.address.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNResponse {
    pub(crate) return_code: String,
    pub(crate) results: Option<Vec<RegionalSDName>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DLRequest {
    pub(crate) station: Option<SDName>,
    pub(crate) stations: Option<Vec<SDName>>,
    pub(crate) time: GTITime,
    pub(crate) max_list: u16,
    pub(crate) max_time_offset: u16,
    pub(crate) all_stations_in_changing_node: bool,
    pub(crate) return_filters: bool,
    pub(crate) filter: Option<Vec<FilterEntry>>,
    pub(crate) service_types: Option<Vec<String>>, // TODO: Use enums!
    pub(crate) use_realtime: bool,
    pub(crate) coordinate_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DLResponse {
    pub time: GTITime,
    pub departures: Vec<Departure>,
    pub filter: Option<Vec<FilterEntry>>,
    pub service_types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub line: Service,
    pub direction: u8, // 0 hinfahrt, 6 rückfahrt
    pub time_offset: i16,
    pub station: SDName,
    pub stop_point: SDName,
    pub service_id: u16,
    pub platform: String,
    pub delay: i16, // delay time in seconds
    pub extra: bool,
    pub cancelled: bool,
    pub realtime_platform: Option<String>,
    pub vehicles: Vec<Vehicle>,
    pub attributes: Option<Attribute>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
    pub id: Option<String>,
    pub number: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub value: String,
    pub types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: Option<String>,
    pub name: Option<String>,
    pub direction: Option<String>,
    pub direction_id: Option<u8>,
    #[serde(rename = "type")]
    pub service_type: Option<ServiceType>,
    pub carrier_name_short: Option<String>,
    pub carrier_name_long: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterEntry {
    #[serde(rename = "serviceID")]
    pub service_id: String,
    #[serde(rename = "stationIDs")]
    pub stations_ids: Vec<String>,
    pub service_name: Option<String>,
    pub label: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct GTITime {
    pub time: String,
    pub date: String
}
