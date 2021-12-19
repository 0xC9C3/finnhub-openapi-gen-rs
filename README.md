# finnhub-openapi-gen-rs

Basic script to generate an api lib for [finnhub.io](https://finnhub.io) using the openapi schema.

You can change the generator settings in gen.sh using the [openapi rust generator](https://openapi-generator.tech/docs/generators/rust)

Most endpoints are not tested, so please be careful before using this project in production.

Example:

```rust
use finnhub_openapi_gen_rs::apis::configuration::{ApiKey, Configuration};
use finnhub_openapi_gen_rs::apis::default_api::stock_symbols;

#[tokio::main]
async fn main() {
    let api_key = Some(
        ApiKey {
            prefix: None,
            key: "YOUR_API_KEY".to_string(),
        }
    );
    let configuration = Configuration {
        api_key,
        ..Configuration::default()
    };

    let symbols = stock_symbols(
        &configuration,
        "US",
        None,
        None,
        None,
    ).await;
    println!("{:?}", symbols);
}

```