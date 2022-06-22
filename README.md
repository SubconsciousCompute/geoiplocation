# geoiplocation
Get location data for a given IP

Find location given the IP address.

Provides abstraction over the fields returned by the JSON response. Returns a `Struct` for the
given query.

You can set your own env for location api using something like `export LOCATION_API_URL=<your-url>`
The final URL looks something like `your-url/IP?apikey=KEY`.

Two methods are provides(more documentation provided):

- `async fn get_location(ip: &str, key: &str) -> anyhow::Result<Option<Location>>`
- `async fn get_location_fallback(ip: &str, key: &str) -> anyhow::Result<HashMap<String, String>>`

**NOTE:** Still in production so a few things might break or change, you can use `cargo doc --open` for better
experience or read it [here](https://docs.rs/geoiplocation/latest/geoiplocation/index.html).

Eg:

```rust
use geoiplocation::get_location;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get_location(
        "8.8.8.8".parse().unwrap(),
        "YOUR-KEY",
    )
    .await?;
    println!("{:?}", resp);
    
    Ok(())
}
```

Output:

```
Some(
    Location {
        ip: Some(
            "8.8.8.8",
        ),
        city: Some(
            "",
        ),
        country: Some(
            "United States",
        ),
        continent: Some(
            "North America",
        ),
    },
)

```
