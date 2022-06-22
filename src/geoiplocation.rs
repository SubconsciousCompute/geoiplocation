use serde::Deserialize;

/// Fields of the JSON response
/// Everything is an `Option` in case it returns wrong or empty values
///
/// You can also use `get_location_fallback` in case the JSON response changes
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    /// The ip we ask it to get data for
    ip: Option<String>,
    /// Name of the city the IP could be located at
    city: Option<String>,
    /// Name of the country the city is in
    country: Option<String>,
    /// Name of the continent the city is in
    continent: Option<String>,
}
