use reqwest;
use reqwest::Url;

use crate::errors::{ErrorResponse, PlaidError, Result};

#[derive(Debug, Copy, Clone)]
pub enum Environment {
    Sandbox,
    Development,
    Production,
}

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    pub client_id: String,
    pub secret: String,
    environment: Environment,
}

impl Client {
    /// Create a plaid client using the supplied credentials.
    ///
    /// * `client_id` - Your Plaid API client_id.
    /// * `secret` - Your Plaid API secret.
    /// * `environment` - The Plaid environment to use.
    pub fn new(client_id: String, secret: String, environment: Environment) -> Client {
        Client {
            client: reqwest::Client::builder().build().unwrap(),
            client_id,
            secret,
            environment,
        }
    }

    /// Create a plaid client using credentials supplied from the environment.
    ///
    /// Credentials must be passed in `PLAID_CLIENT_ID`, `PLAID_SECRET` and `PLAID_ENVIRONMENT` environment variables.
    /// `PLAID_ENVIRONMENT` must be set to `SANDBOX`, `DEVELOPMENT` or `PRODUCTION`.
    pub fn from_env() -> Client {
        let plaid_environment = match &std::env::var("PLAID_ENVIRONMENT")
        .expect("Missing environment variable: PLAID_ENVIRONMENT. Must be set to SANDBOX, DEVELOPMENT or PRODUCTION")
        .to_string().to_lowercase()[..] {
            "SANDBOX" => Environment::Sandbox,
            "DEVELOPMENT" => Environment::Development,
            "PRODUCTION" => Environment::Production,
            s => panic!("Environment variable PLAID_ENVIRONMENT must be set to SANDBOX, DEVELOPMENT or PRODUCTION. Is actually {}", s),
        };

        Client::new(
            std::env::var("PLAID_CLIENT_ID")
                .expect("Missing environment variable: PLAID_CLIENT_ID")
                .to_string(),
            std::env::var("PLAID_SECRET")
                .expect("Missing environment variable: PLAID_SECRET")
                .to_string(),
            plaid_environment,
        )
    }

    pub async fn send_request<T, U>(&self, url: &str, req: &T) -> Result<U>
    where
        T: serde::Serialize,
        U: for<'de> serde::Deserialize<'de>,
    {
        let resp = self
            .client
            .post(self.get_host().join(url).unwrap())
            .json(req)
            .send()
            .await?;
        if resp.status() == reqwest::StatusCode::OK {
            Ok(resp.json().await?)
        } else {
            let status_code = resp.status();
            let err_resp: ErrorResponse = resp.json().await?;
            Err(PlaidError {
                request_id: err_resp.request_id,
                error_type: err_resp.error_type,
                error_code: err_resp.error_code,
                error_message: err_resp.error_message,
                display_message: err_resp.display_message,
                status_code: status_code,
            }
            .into())
        }
    }

    fn get_host(&self) -> Url {
        match self.environment {
            Environment::Sandbox => Url::parse("https://sandbox.plaid.com/").unwrap(),
            Environment::Development => Url::parse("https://development.plaid.com/").unwrap(),
            Environment::Production => Url::parse("https://production.plaid.com/").unwrap(),
        }
    }
}

pub mod tests {
    use super::*;

    pub const SANDBOX_INSTITUTION: &str = "ins_109508";
    pub const SANDBOX_INSTITUTION_QUERY: &str = "Platypus";
    pub const TEST_PRODUCTS: &[&str] = &["auth", "identity", "transactions"];

    pub fn get_test_client() -> Client {
        Client::new(
            std::env::var("PLAID_CLIENT_ID")
                .expect("Missing environment variable: PLAID_CLIENT_ID")
                .to_string(),
            std::env::var("PLAID_SECRET")
                .expect("Missing environment variable: PLAID_SECRET")
                .to_string(),
            Environment::Sandbox,
        )
    }
}
