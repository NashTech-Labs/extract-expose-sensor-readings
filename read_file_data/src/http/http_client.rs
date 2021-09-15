use std::collections::HashMap;
use crate::utils::decode::decode;

static SENSOR_KEY: &str = "sensor";
static SENSOR_NAME: &str = "magnetometer";
static READING: &str = "reading";
static SUCCESS_MESSAGE: &str = "Successfully posted to server";

/// Post data into the server
///
/// # Arguments
///
/// * `data` - data that needs to be posted
///
/// * `url` - URL of the server
///
/// # Return
///
/// This function returns either success message or error message
pub fn post_request(data: &[u8], url: &str) -> String {
    let mut map = HashMap::new();
    map.insert(SENSOR_KEY, SENSOR_NAME);

    match std::str::from_utf8(&decode(data)) {
        Ok(reading) => {
            map.insert(READING, reading);
            let client = reqwest::blocking::Client::new();
            match client.post(url)
                .json(&map).send() {
                Ok(_) => SUCCESS_MESSAGE.to_string(),
                Err(http_error) => http_error.to_string()
            }
        }
        Err(utf8_error) => utf8_error.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::http::http_client::post_request;

    static SERVER_URL: &str = "https://connect.cloud.kaaiot.com:443/kp1/c4hnc5ad4ks1slmoohgg-v1/epmx/magnetometer/update/keys";
    static INVALID_SERVER_URL: &str = "https://xyz/test";

    static DATA: &str = "{ x: 579, y: -197 , z: -485 }\0";

    #[test]
    fn test_post_request_success() {
        assert_eq!(post_request(DATA.as_bytes(), SERVER_URL), "Successfully posted to server".to_string());
    }

    #[test]
    fn test_post_request_http_failure() {
        assert!(post_request(DATA.as_bytes(), INVALID_SERVER_URL).contains("error"));
    }
}
