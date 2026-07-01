use crate::geofox_models::{CNRequest, LSRequest, LSResponse, PCRequest, PCResponse};
use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sha1::Sha1;
use std::str::FromStr;

mod geofox_models;
pub mod model;
pub mod timetable;
pub struct Config {
    geofox_user: String,
    geofox_secret: String,
    geofox_url: String,
}

// Function should receive a json string slice and a password slide, hash it according to geofox and return a base64 encoded string.
fn hash_body_and_password(body: &str, password: &str) -> Result<String> {
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(password.as_ref())?;

    mac.update(body.as_ref());

    let result = mac.finalize().into_bytes();

    let encoded_string = general_purpose::STANDARD.encode(result);

    Ok(encoded_string)
}

// Helper function that constructs the Geofox Request Header
fn build_auth_header(body: &str, user: &str, pw: &str) -> Result<HeaderMap> {
    let mut header = HeaderMap::new();

    let hash_pw = hash_body_and_password(body, pw)?;

    header.append(
        HeaderName::from_str("geofox-auth-user")?,
        HeaderValue::from_str(user)?,
    );
    header.append(
        HeaderName::from_str("geofox-auth-signature")?,
        HeaderValue::from_str(&hash_pw)?,
    );
    header.append(
        HeaderName::from_str("geofox-auth-type")?,
        HeaderValue::from_str("HmacSHA1")?,
    );
    header.append(
        HeaderName::from_str("Content-Type")?,
        HeaderValue::from_str("application/json;charset=UTF-8")?,
    );

    Ok(header)
}

// Calls the /init endpoint -> can be used to verify credentials and query information about the Geofox service
/// Function to call the init endpoint of the geofox api
///
/// This function can be used to check if your API credentials are valid and gather some basic information about the API
///
/// # Arguments
/// * `cfg` - `&Config` object with the API credentials configuration.
///
/// # Returns
/// * A `Result<reqwest::Response>`. Will return an `Err` if anything went wrong.
pub async fn init(cfg: &Config) -> Result<Response> {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/init");
    let client = reqwest::Client::new();

    let header = build_auth_header("{}", &*cfg.geofox_user, &*cfg.geofox_secret)?;

    let res = client.post(url).headers(header).body("{}").send().await?;

    Ok(res)
}

/// Function to check if a postal code is inside the hvv service area.
///
/// This method calls the /checkPostalCode endpoint
///
/// # Arguments
/// `cfg` - `&Config` object with the API credentials configuration.
/// `postal_code` - `u16` that includes the 5-digit postal code used in germany
///
/// # Returns
/// * A Result<bool> if the postal code is inside the service area. Returns an `Err` if anything went wrong.
pub async fn check_postal_code(cfg: &Config, postal_code: u16) -> Result<bool> {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/checkPostalCode ");
    let client = reqwest::Client::new();

    let body = PCRequest {
        postalCode: postal_code,
        version: 63,
    };
    let ser = serde_json::to_string(&body).unwrap();

    let header = build_auth_header(&ser, &*cfg.geofox_user, &*cfg.geofox_secret)?;

    let res = client.post(url).headers(header).body(ser).send().await?;

    let response_message: PCResponse = serde_json::from_str(&res.text().await?)?; // this pretty sure cannot not panic

    Ok(response_message.isHVV)
}

// Check if a name exists and return the station: /gti/public/checkName
pub fn check_name(
    cfg: &Config,
    search_name: &str,
    max_search: u16,
    max_dist: u16,
    include_tariff_details: bool,
    allow_type_switch: bool,
) -> Result<String, String> {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/checkName");
    let client = reqwest::Client::new();

    let body = CNRequest {
        theName: search_name.to_string(),
        maxListL: max_search,
        maxDistance: max_dist,
        coordinateType: "EPSG_4326".to_string(),
        tariffDetails: include_tariff_details,
        allowTypeSwitch: allow_type_switch,
    };

    let body_str = match serde_json::to_string(&body) {
        Ok(ser) => ser,
        Err(_) => return Err("Could not serialize body".to_string()),
    };

    //let header = build_auth_header();
    Err("Function not implemented".to_string())
}

// TODO: Convert to better types with all information
/// Function that calls the /gti/public/listStations endpoints to prefetch all stations.
///
/// This can be used for caching or prefetching all currently available stations
///
/// # Arguments
/// * `cfg` - `&Config` object with the API credentials configuration.
/// * `filter_equivalent_statons` - `bool` that marks if it should filter stations that are equivalent (e.g. Rödingsmarkt and Rödingsmarkt U)
/// * `data_release_date` - `&str` that can eather be empty to fetch all data or can include a timestamp from the last fetch (to only fetch new data)
///
/// # Returns
/// * A Vector of type `Result<Vec<geofox_models::StationListEntry>>` with all fetched stations without any cleanups
pub async fn list_stations(
    cfg: &Config,
    filter_equivalent_stations: bool,
    data_release_date: &str,
) -> Result<Vec<geofox_models::StationListEntry>> {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/listStations");
    let client = reqwest::Client::new();

    let body = LSRequest {
        dataReleaseID: data_release_date.to_string(),
        modificationTypes: vec!["MAIN".to_string()],
        coordinateType: "EPSG_4326".to_string(),
        filterEquivalent: filter_equivalent_stations,
    };

    let serialized_body = serde_json::to_string(&body).unwrap(); // known good

    println!("{}", serialized_body);

    let header = build_auth_header(&serialized_body, &cfg.geofox_user, &cfg.geofox_secret)?;

    let response = match client
        .post(url)
        .headers(header)
        .body(serialized_body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(_) => panic!("Failed to get stations list"),
    };

    let response_code = response.status();
    println!("{}", response_code);

    let json_string = response.text().await?;

    let data: LSResponse = serde_json::from_str(&json_string)?;

    Ok(data.stations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn test_body_password_encoder() {
        let body_json_str = "{\"test\": \"top\"}";

        let password = "password";

        let hashed_string = hash_body_and_password(body_json_str, password).unwrap();

        assert_eq!(hashed_string, "NluTzK4mYLMjBg6YRe6ROJM9ZuI=");
    }

    fn build_config() -> Config {
        dotenv().ok();

        let pw = env::var("GEOFOX_SECRET").expect("PW MUST BE SET");
        let user = env::var("GEOFOX_USER").expect("USER MUST BE SET");
        let url = env::var("GEOFOX_BASEURL").expect("URL MUST BE SET");

        Config {
            geofox_user: user,
            geofox_secret: pw,
            geofox_url: url,
        }
    }

    #[tokio::test]
    async fn test_geofox_http_header_login() {
        let config = build_config();

        let response = init(&config).await.unwrap();
        println!("{}", response.status());
        assert!(response.status().is_success());
        println!("{}", response.text().await.unwrap());
    }

    #[tokio::test]
    async fn test_check_postal_function() {
        let config = build_config();

        let should_not_exist = check_postal_code(&config, 12345).await.unwrap();
        let should_exist = check_postal_code(&config, 20097).await.unwrap();

        assert!(should_exist);
        assert_eq!(should_not_exist, false);
    }

    #[tokio::test]
    async fn test_list_stations_function() {
        let config = build_config();

        let stations = list_stations(&config, false, "").await.unwrap();

        assert_eq!(stations.is_empty(), false); // the list should not be empty!
    }

    #[tokio::test]
    async fn test_get_name_function() {
        let config = build_config();

        let results = check_name(&config, "Altona", 1, 3000, false, false).unwrap();
    }
}
