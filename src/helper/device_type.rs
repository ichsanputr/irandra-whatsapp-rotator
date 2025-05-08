use actix_web::HttpRequest;

pub fn get_device_type(req: &HttpRequest) -> String {
    // Get User-Agent header safely
    let ua_str = match req.headers().get("User-Agent") {
        Some(user_agent) => match user_agent.to_str() {
            Ok(str) => str,
            Err(_) => return "Unknown".to_string(), // If there's an error converting to str
        },
        None => return "Unknown".to_string(), // If User-Agent header is missing
    };

    // Determine device type based on User-Agent string
    if ua_str.to_lowercase().contains("mobile")
        || ua_str.to_lowercase().contains("android")
        || ua_str.to_lowercase().contains("iphone")
        || ua_str.to_lowercase().contains("ipad")
    {
        "Mobile".to_string()
    } else {
        "Desktop".to_string()
    }
}