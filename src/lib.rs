//! Find location given the IP address.
//!
//! Provides abstraction over the fields returned by the JSON response. Returns a `Struct` for the
//! given query.
//! You can set your own env for location api using something like `export LOCATION_API_URL=<your-url>`
//! The final URL looks something like `your-url/IP?apikey=KEY`

use crate::geoiplocation::Location;
use reqwest::Url;

pub mod geoiplocation;
use std::env;

/// `async` function to get the location
///
/// Eg:
/// ```rust
/// use geoiplocation::get_location;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let resp = get_location(
///         "8.8.8.8".parse().unwrap(),
///         "YOUR-KEY",
///     )
///     .await?;
///     println!("{:#?}", resp);
///     // Some(Location { ip: Some("8.8.8.8"), city: Some(""), country: Some("United States"), continent: Some("North America") })
///
///     Ok(())
/// }
/// ```
pub async fn get_location(
    ip: &str,
    key: &str,
) -> Result<Option<Location>, Box<dyn std::error::Error>> {
    let location_api_url = env::var("LOCATION_API_URL").expect("cannot parse URL");

    let mut url = Url::parse(&*format!("{}/{}", location_api_url, ip))?;
    url.set_query(Some(&*format!("apikey={}", key)));

    Ok(Some(reqwest::get(url).await?.json::<Location>().await?))
}
