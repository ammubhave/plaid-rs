use serde::Deserialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub request_id: String,
    pub error_type: String,
    pub error_code: String,
    pub error_message: String,
    pub display_message: Option<String>,
}

#[derive(Debug)]
pub enum Error {
    /// Error returned by the Plaid API
    Plaid(PlaidError),
    /// Error when sending request
    Request(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error - {}",
            match self {
                Self::Plaid(err) => err.to_string(),
                Self::Request(err) => err.to_string(),
            },
        )
    }
}

impl From<PlaidError> for Error {
    fn from(err: PlaidError) -> Self {
        Self::Plaid(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

#[derive(Debug, Clone)]
pub struct PlaidError {
    /// A broad categorization of the error. Safe for programatic use.
    /// Possible values: INVALID_REQUEST, INVALID_INPUT, INSTITUTION_ERROR, RATE_LIMIT_EXCEEDED, API_ERROR, ITEM_ERROR, ASSET_REPORT_ERROR, RECAPTCHA_ERROR, OAUTH_ERROR, PAYMENT_ERROR, BANK_TRANSFER_ERROR
    pub error_type: String,
    /// The particular error code. Safe for programmatic use.
    pub error_code: String,
    /// A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    /// A user-friendly representation of the error code. None if the error is not related to user action.
    /// This may change over time and is not safe for programmatic use.
    pub display_message: Option<String>,
    /// A unique identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    pub request_id: String,
    /// The HTTP status code associated with the error.
    pub status_code: reqwest::StatusCode,
}

impl fmt::Display for PlaidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Plaid Error - request ID: {}, http status: {}, type: {}, code: {}, message: {}, display_message: {}",
            self.request_id,
            self.status_code,
            self.error_type,
            self.error_code,
            self.error_message,
            self.display_message.as_ref().unwrap_or(&"".to_string()),
        )
    }
}
