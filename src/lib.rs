use crate::geofox_models::{PCRequest, PCResponse};
use base64::Engine;
use base64::engine::general_purpose;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
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
fn hash_body_and_password(body: &str, password: &str) -> String {
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(password.as_ref()).expect("Key import must work");

    mac.update(body.as_ref());

    let result = mac.finalize().into_bytes();

    let encoded_string = general_purpose::STANDARD.encode(result);

    encoded_string
}

// Helper function that constructs the Geofox Request Header
fn build_auth_header(body: &str, user: &str, pw: &str) -> HeaderMap {
    let mut header = HeaderMap::new();

    let hash_pw = hash_body_and_password(body, pw);

    header.append(
        HeaderName::from_str("geofox-auth-user").unwrap(),
        HeaderValue::from_str(user).unwrap(),
    );
    header.append(
        HeaderName::from_str("geofox-auth-signature").unwrap(),
        HeaderValue::from_str(&hash_pw).unwrap(),
    );
    header.append(
        HeaderName::from_str("geofox-auth-type").unwrap(),
        HeaderValue::from_str("HmacSHA1").unwrap(),
    );
    header.append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str("application/json;charset=UTF-8").unwrap(),
    );

    header
}

// Calls the /init endpoint -> can be used to verify credentials and query information about the Geofox service
pub async fn init(cfg: &Config) -> Response {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/init");
    let client = reqwest::Client::new();

    let header = build_auth_header("{}", &*cfg.geofox_user, &*cfg.geofox_secret);

    let res = match client.post(url).headers(header).body("{}").send().await {
        Ok(resp) => resp,
        Err(_) => panic!("Couldnt contact client!"),
    };

    res
}

// Calls the /checkPostalCode endpoint to validate if a given postal code is inside the HVV services area
pub async fn check_postal_code(cfg: &Config, postal_code: u16) -> bool {
    let url = format!("{}{}", cfg.geofox_url, "/gti/public/checkPostalCode ");
    let client = reqwest::Client::new();

    let body = PCRequest {
        postalCode: postal_code,
        version: 63,
    };
    let ser = serde_json::to_string(&body).unwrap();

    let header = build_auth_header(&ser, &*cfg.geofox_user, &*cfg.geofox_secret);

    let res = match client.post(url).headers(header).body(ser).send().await {
        Ok(res) => res,
        Err(_) => panic!("CheckPostalCode Request failed"),
    };

    let response_message: PCResponse = serde_json::from_str(&res.text().await.unwrap()).unwrap();

    response_message.isHVV
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

        let hashed_string = hash_body_and_password(body_json_str, password);

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

        let response = init(&config).await;
        println!("{}", response.status());
        assert!(response.status().is_success());
        println!("{}", response.text().await.unwrap());
    }

    #[tokio::test]
    async fn test_check_postal_function() {
        let config = build_config();

        let should_not_exist = check_postal_code(&config, 12345).await;
        let shoud_exist = check_postal_code(&config, 20097).await;

        assert!(shoud_exist);
        assert_eq!(should_not_exist, false);
    }
}
