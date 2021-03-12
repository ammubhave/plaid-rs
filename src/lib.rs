//! Rust async client library for accessing the [Plaid API](https://plaid.com/docs/api/).
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! plaid = "0.3"
//! ```
//!
//! To make API calls, you need to create an instance of the Plaid **[`Client`][client]**. The client can be created by calling `plaid::Client::new(client_id, secret, environment)`, or by calling `plaid::Client::from_env()` and passing the credentials in `PLAID_CLIENT_ID`, `PLAID_SECRET`, and `PLAID_ENVIRONMENT` environment variables.
//!
//! ## Examples
//!
//! The following example shows you how to connect to Plaid, and retrieve transactions:
//!
//! ```ignore
//! use plaid::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Get a valid sandbox access token. You should substitute `access_token` with your own valid access token.
//!     let sandbox_resp = client
//!         .create_sandbox_public_token("ins_109508", &["auth", "identity", "transactions"])
//!         .await
//!         .unwrap();
//!     let token_resp = client
//!         .exchange_public_token(&sandbox_resp.public_token)
//!         .await
//!         .unwrap();
//!     let access_token = token_resp.access_token;
//!
//!     let end_date = Utc::now().naive_utc().date();
//!     let start_date = end_date.sub(chrono::Duration::days(365));
//!
//!     let resp = client
//!         .get_transactions(&access_token, start_date, end_date, None)
//!         .await
//!         .unwrap();
//!     for transaction in &resp.transactions {
//!         println!("{:?}", transaction);
//!     }
//! }
//! ```
//!
//! ## Testing
//!
//! You can run `cargo test` to run the test suite. You must supply sandbox credentials in `PLAID_CLIENT_ID` and `PLAID_SECRET` environment variables, or the tests will fail.
//!

pub mod accounts;
pub mod auth;
pub mod categories;
pub mod client;
pub mod deposit_switch;
pub mod errors;
pub mod holdings;
pub mod identity;
pub mod institutions;
pub mod investment_transactions;
pub mod item;
pub mod liabilities;
pub mod link_token;
pub mod processor;
pub mod sandbox;
pub mod transactions;
pub mod webhooks;

pub use client::Client;
