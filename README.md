# plaid-rs

![Build Status](https://github.com/ammubhave/plaid-rs/actions/workflows/rust.yml/badge.svg)
[![](http://meritbadge.herokuapp.com/plaid)](https://crates.io/crates/plaid)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Rust async client library for accessing the [Plaid API](https://plaid.com/docs/api/).

## Documentation

Please see the [documentation website](https://ammubhave.github.io/plaid-rs), or at [docs.rs](https://docs.rs/plaid/).

The source code can be found at [https://github.com/ammubhave/plaid-rs](https://github.com/ammubhave/plaid-rs). Source code for older versions (<= 0.2.0) of this crate can found at [https://github.com/nathankot/plaid-rust](https://github.com/nathankot/plaid-rust).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
plaid = "0.3"
```

To make API calls, you need to create an instance of the Plaid Client. The client can be created by calling `plaid::Client::new(client_id, secret, environment)`, or by calling `plaid::Client::from_env()` and passing the credentials in `PLAID_CLIENT_ID`, `PLAID_SECRET`, and `PLAID_ENVIRONMENT` environment variables.

## Examples

The following example shows you how to connect to Plaid, and retrieve transactions:

```rust
use plaid::Client;

#[tokio::main]
async fn main() {
    // Get a valid sandbox access token. You should substitute `access_token` with your own valid access token.
    let sandbox_resp = client
        .create_sandbox_public_token("ins_109508", &["auth", "identity", "transactions"])
        .await
        .unwrap();
    let token_resp = client
        .exchange_public_token(&sandbox_resp.public_token)
        .await
        .unwrap();
    let access_token = token_resp.access_token;

    let end_date = Utc::now().naive_utc().date();
    let start_date = end_date.sub(chrono::Duration::days(365));

    let resp = client
        .get_transactions(&access_token, start_date, end_date, None)
        .await
        .unwrap();
    for transaction in &resp.transactions {
        println!("{:?}", transaction);
    }
}
```

## Testing

You can run `cargo test` to run the test suite. You must supply sandbox credentials in `PLAID_CLIENT_ID` and `PLAID_SECRET` environment variables, or the tests will fail.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](LICENSE).

## Disclaimer

This SOFTWARE PRODUCT is provided by THE PROVIDER "as is" and "with all faults."
THE PROVIDER makes no representations or warranties of any kind concerning the
safety, suitability, lack of viruses, inaccuracies, typographical errors, or
other harmful components of this SOFTWARE PRODUCT. There are inherent dangers
in the use of any software, and you are solely responsible for determining
whether this SOFTWARE PRODUCT is compatible with your equipment and other
software installed on your equipment. You are also solely responsible for the
protection of your equipment and backup of your data, and THE PROVIDER will not
be liable for any damages you may suffer in connection with using, modifying,
or distributing this SOFTWARE PRODUCT.
