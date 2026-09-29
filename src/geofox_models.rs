use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCRequest {
    pub version: u8,
    pub postal_code: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PCResponse {
    pub return_code: String,
    #[serde(rename = "isHVV")]
    pub is_hvv: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LSRequest {
    #[serde(rename = "dataReleaseID")]
    pub data_release_id: String,
    pub modification_types: Vec<String>,
    pub coordinate_type: String,
    pub filter_equivalent: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LSResponse {
    pub return_code: String,
    #[serde(rename = "dataReleaseID")]
    pub data_release_id: String,
    pub stations: Option<Vec<StationListEntry>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LLRequest {
    #[serde(rename = "dataReleaseID")]
    pub data_release_id: String,
    pub modification_types: Vec<String>,
    pub with_sublines: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LLResponse {
    #[serde(rename = "dataReleaseID")]
    pub data_release_id: String,
    pub lines: Option<Vec<LineListEntry>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineListEntry {
    pub id: String,
    pub name: Option<String>,
    pub carrier_name_short: Option<String>,
    pub carrier_long_name: Option<String>,
    pub sublines: Option<Vec<SublineListEntry>>,
    pub exists: Option<bool>,
    #[serde(rename = "type")]
    pub service_type: Option<ServiceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceType {
    pub simple_type: String,
    pub short_info: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SublineListEntry {
    pub subline_number: String,
    pub vehicle_type: String,
    pub station_sequence: Vec<StationLight>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationLight {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationListEntry {
    pub id: String,
    pub name: Option<String>,
    pub city: Option<String>,
    pub combined_name: Option<String>,
    pub shortcuts: Option<Vec<String>>,
    pub aliasses: Option<Vec<String>>,
    pub vehicle_types: Option<Vec<String>>,
    pub coordinate: Option<Coordinate>,
    pub exists: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CNRequest {
    pub the_name: SDName,
    pub max_list_l: u16,
    pub max_distance: u16,
    pub coordinate_type: String,
    pub tariff_details: bool,
    pub allow_type_switch: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Coordinate {
    x: f32,
    y: f32,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SDName {
    pub name: Option<String>,
    pub city: Option<String>,
    pub combined_name: Option<String>,
    #[serde(rename = "type")]
    pub sd_type: Option<String>,
    pub coordinate: Option<Coordinate>,
    pub layer: Option<i16>,
    pub tariff_details: Option<TariffDetail>,
    pub has_station_information: Option<bool>,
    pub provider: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TariffDetail {
    pub inner_city: Option<String>,
    pub city_traffic: Option<String>,
    pub gratis: Option<bool>,
    pub greater_area: Option<bool>,
    pub sh_village_id: Option<i16>,
    pub sh_tariff_zones: Option<Vec<i16>>,
    pub tariff_zones: Option<Vec<i16>>,
    pub counties: Option<Vec<String>>,
    pub rings: Option<Vec<String>>,
    pub fare_stage: Option<bool>,
    pub fare_stage_number: Option<i16>,
    pub tariff_names: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalSDName {
    pub name: Option<String>,
    pub city: Option<String>,
    pub combined_name: Option<String>,
    #[serde(rename = "type")]
    pub sd_type: Option<String>,
    pub coordinate: Option<Coordinate>,
    pub layer: Option<i16>,
    pub tariff_details: Option<TariffDetail>,
    pub has_station_information: Option<bool>,
    pub provider: Option<String>,
    pub address: Option<String>,
    pub distance: Option<i32>,
    pub time: Option<String>,
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
    pub return_code: String,
    pub results: Option<Vec<RegionalSDName>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DLRequest {
    pub station: Option<SDName>,
    pub stations: Option<Vec<SDName>>,
    pub time: GTITime,
    pub max_list: u16,
    pub max_time_offset: u16,
    pub all_stations_in_changing_node: bool,
    pub return_filters: bool,
    pub filter: Option<Vec<FilterEntry>>,
    pub service_types: Option<Vec<String>>, // TODO: Use enums!
    pub use_realtime: bool,
    pub coordinate_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DLResponse {
    pub time: GTITime,
    pub departures: Vec<Departure>,
    pub filter: Option<Vec<FilterEntry>>,
    pub service_types: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub line: Service,
    pub direction: Option<u8>, // 0 hinfahrt, 6 rückfahrt
    pub time_offset: i16,
    pub station: Option<SDName>,
    pub stop_point: Option<SDName>,
    pub service_id: Option<u16>,
    pub platform: Option<String>,
    pub delay: Option<i16>, // delay time in seconds
    pub extra: Option<bool>,
    pub cancelled: Option<bool>,
    pub realtime_platform: Option<String>,
    pub vehicles: Option<Vec<Vehicle>>,
    pub attributes: Option<Attribute>,
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
    pub carrier_name_long: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterEntry {
    #[serde(rename = "serviceID")]
    pub service_id: String,
    #[serde(rename = "stationIDs")]
    pub stations_ids: Vec<String>,
    pub service_name: Option<String>,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GTITime {
    pub time: String,
    pub date: String,
}

impl GTITime {
    pub fn from_chronos_time(
        chronos_time: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<GTITime> {
        let chronos_time_europe = chronos_time.with_timezone(&chrono_tz::Europe::Berlin);
        let date = chronos_time_europe
            .date_naive()
            .format("%d.%m.%Y")
            .to_string();
        let time = chronos_time_europe.time().format("%H:%M").to_string();

        Ok(GTITime { time, date })
    }
}
