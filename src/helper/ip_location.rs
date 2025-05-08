use serde::Deserialize;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct IpLocation {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub regionName: Option<String>,
    pub city: Option<String>,
    pub status: String,
}

pub async fn get_location_from_ip(ip: &str) -> (String, String) {
    let url = format!("http://ip-api.com/json/{}", ip);

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<IpLocation>().await {
            Ok(location) => {
                if location.status == "success" {
                    let city = location.city.unwrap_or_else(|| "-".to_string());
                    let region = location.regionName.unwrap_or_else(|| "-".to_string());
                    let country = location.country.unwrap_or_else(|| "-".to_string());

                    if let (Some(lat), Some(lon)) = (location.lat, location.lon) {
                        let gmaps_link = format!("https://www.google.com/maps?q={},{}", lat, lon);
                        (format!("{}, {}, {}", city, region, country), gmaps_link)
                    } else {
                        ("".to_string(), "".to_string())
                    }
                } else {
                    ("".to_string(), "".to_string())
                }
            }
            Err(err) => {
                error!("Error add report: {}", err);
                ("Error parsing location".to_string(), "".to_string())
            }
        },
        Err(err) => {
            error!("Error add report: {}", err);
            ("Error fetching location".to_string(), "".to_string())
        }
    }
}
