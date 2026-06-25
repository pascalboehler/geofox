use std::str::FromStr;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use hmac::{Hmac, KeyInit, Mac};
use sha1::Sha1;
use base64::engine::general_purpose;

pub mod model;
pub mod timetable;

pub struct Config {
    geofox_user: String,
    geofox_secret: String,
    geofox_url: String,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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

    header.append(HeaderName::from_str("geofox-auth-user").unwrap(),
                  HeaderValue::from_str(user).unwrap());
    header.append(HeaderName::from_str("geofox-auth-signature").unwrap(),
                  HeaderValue::from_str(&hash_pw).unwrap());
    header.append(HeaderName::from_str("geofox-auth-type").unwrap(),
                  HeaderValue::from_str("HmacSHA1").unwrap());
    header.append(HeaderName::from_str("Content-Type").unwrap(),
                  HeaderValue::from_str("application/json;charset=UTF-8").unwrap());

    header
}

pub async fn init(cfg: Config) -> Response {
    let url = cfg.geofox_url + "/gti/public/init";
    let client = reqwest::Client::new();

    let header = build_auth_header("{}", &*cfg.geofox_user, &*cfg.geofox_secret);

    let res = match client.post(url)
        .headers(header)
        .body("{}")
        .send()
        .await {
        Ok(resp) => resp,
        Err(resp) => panic!("Couldnt contact client!")
    };

    res
}

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::*;

    #[test]
    fn test_body_password_encoder() {
        let body_json_str = "{\"test\": \"top\"}";

        let password = "password";

        let hashed_string = hash_body_and_password(body_json_str, password);

        assert_eq!(hashed_string, "NluTzK4mYLMjBg6YRe6ROJM9ZuI=");

    }

    #[tokio::test]
   async fn test_geofox_http_header_login() {

        dotenv().ok();

        let pw = env::var("GEOFOX_SECRET").expect("PW MUST BE SET");
        let user = env::var("GEOFOX_USER").expect("USER MUST BE SET");
        let url = env::var("GEOFOX_BASEURL").expect("URL MUST BE SET");

        let config = Config{
            geofox_user: user,
            geofox_secret: pw,
            geofox_url: url
        };

        let response = init(config).await;

        assert!(response.status().is_success());
        println!("{}", response.text().await.unwrap());
    }
}
