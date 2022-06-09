# geoiplocation
Get location data for a given IP

Find location given the IP address.Provides abstraction over the fields returned by the JSON response. Returns a `Struct` for the given query.

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
