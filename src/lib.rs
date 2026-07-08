use crate::geofox_models::{CNRequest, CNResponse, LSRequest, LSResponse, PCRequest, PCResponse, RegionalSDName, SDName};
use anyhow::Result;
use base64::{encode, Engine};
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

/// Private helper function that calculates the required authentication hash
///
/// The GTI Geofox API requires the user to calculate a base64 encoded Sha1 HMAC Challenge using the password and encoded json body for authentication.
/// This function calculates this header according to the APIs requirements to simplify the authentication process.
/// This will be used by the crate during the construction of the http header.
///
/// # Arguments
/// * `body` - `&str` with the serialized json request body.
/// * `password` - `&str` with the assigned api password.
///
/// # Returns
/// * `Result<String>` - The hashed and base64 encoded result string. Will return an `Err` if anything goes wrong.
fn hash_body_and_password(body: &str, password: &str) -> Result<String> {
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(password.as_ref())?;

    mac.update(body.as_ref());

    let result = mac.finalize().into_bytes();

    let encoded_string = general_purpose::STANDARD.encode(result);

    Ok(encoded_string)
}

/// Helper function to construct the correct http header
///
/// The Geofox API requires the user to construct a special http header for the authentication to work based on your credentials and the actual request body.
///
/// # Arguments
/// * `body` - `&str` that includes the serialized json string
/// * `user` - `&str` with the assigned username
/// * `pw` - `&str` the assigned geofox api password, used to authenticate against the api
///
/// # Returns
/// - `Result<HeaderMap>` - Returns a reqwest::HeaderMap that can be used for the http request. Will return an `Err` if anything goes wrong
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
        postal_code,
        version: 63,
    };
    let ser = serde_json::to_string(&body)?;

    let header = build_auth_header(&ser, &*cfg.geofox_user, &*cfg.geofox_secret)?;

    let res = client.post(url).headers(header).body(ser).send().await?;

    let response_message: PCResponse = serde_json::from_str(&res.text().await?)?; // this pretty sure cannot not panic

    Ok(response_message.is_hvv)
}

// Check if a name exists and return the station: /gti/public/checkName
pub async fn check_name(
    cfg: &Config,
    search_name: SDName,
    max_search: u16,
    max_dist: u16,
    include_tariff_details: bool,
    allow_type_switch: bool,
) -> Result<Vec<RegionalSDName>> {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/checkName");
    let client = reqwest::Client::new();

    let body = CNRequest {
        the_name: search_name,
        max_list_l: max_search,
        max_distance: max_dist,
        coordinate_type: "EPSG_4326".to_string(),
        tariff_details: include_tariff_details,
        allow_type_switch,
    };

    let body_str = serde_json::to_string(&body)?;
    let header = build_auth_header(&body_str, &cfg.geofox_user, &cfg.geofox_secret)?;

    let resp = client.post(url).headers(header).body(body_str).send().await?;

    let encoded_json = resp.text().await?;
    let data_returned: CNResponse = serde_json::from_str(&encoded_json)?;

    Ok(data_returned.results.unwrap_or_else(|| vec![]))
}

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
        data_release_id: data_release_date.to_string(),
        modification_types: vec!["MAIN".to_string()],
        coordinate_type: "EPSG_4326".to_string(),
        filter_equivalent: filter_equivalent_stations,
    };

    let serialized_body = serde_json::to_string(&body)?; // known good

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

        let search_term = SDName {
            name: Some("Altona".to_string()),
            city: None,
            combined_name: None,
            sd_type: Some("UNKNOWN".to_string()),
            coordinate: None,
            layer: None,
            tariff_details: None,
            has_station_information: None,
            provider: None,
            address: None,
        };

        let results = check_name(&config, search_term, 1, 3000, false, false).await.unwrap();
        assert_eq!(results.is_empty(), false);
    }
}
