use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

#[derive(Serialize)]
struct CreateProcessorTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    account_id: &'a str,
    processor: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateProcessorTokenResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The processor_token that can then be used by the Plaid partner to make API requests
    pub processor_token: String,
}

impl Client {
    /// Create processor token.
    ///
    /// Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see /processor/stripe/bank_account_token/create for creating tokens for use with Stripe integrations.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `account_id` - The account_id value obtained from the onSuccess callback in Link.
    /// * `processor` - The processor you are integrating with. Valid values are "achq", "check", "checkbook", "circle", "drivewealth", "dwolla", "galileo", "interactive_brokers", "modern_treasury", "ocrolus", "prime_trust", "rize", "sila_money", "unit", "velox", "vesta", "vopay", "wyre"
    pub async fn create_processor_token(
        &self,
        access_token: &str,
        account_id: &str,
        processor: &str,
    ) -> Result<CreateProcessorTokenResponse> {
        self.send_request(
            "processor/token/create",
            &CreateProcessorTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                account_id,
                processor,
            },
        )
        .await
    }
}
