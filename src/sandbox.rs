use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

#[derive(Serialize)]
struct CreateSandboxPublicTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    institution_id: &'a str,
    initial_products: &'a [&'a str],
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateSandboxPublicTokenResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// A public token that can be exchanged for an access token using /item/public_token/exchange
    pub public_token: String,
}

#[derive(Serialize)]
struct ResetSandboxItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResetSandboxItemResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// true if the call succeeded
    pub reset_login: bool,
}

#[derive(Serialize)]
struct SetSandboxItemVerificationStatusRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    account_id: &'a str,
    verification_status: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SetSandboxItemVerificationStatusResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}

#[derive(Serialize)]
struct FireWebhookRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    webhook_code: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FireWebhookResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// Value is true  if the test webhook_code  was successfully fired.
    pub webhook_fired: bool,
}

impl Client {
    /// Create a test Item.
    ///
    /// Use the /sandbox/public_token/create endpoint to create a valid public_token for an arbitrary institution ID, initial products, and test credentials. The created public_token maps to a new Sandbox Item. You can then call /item/public_token/exchange to exchange the public_token for an access_token and perform all API actions. /sandbox/public_token/create can also be used with the user_custom test username to generate a test account with custom data.
    ///
    /// * `institution_id` - The ID of the institution the Item will be associated with.
    /// * `initial_products` - The products to initially pull for the Item. May be any products that the specified institution_id  supports. This array may not be empty.
    pub async fn create_sandbox_public_token(
        &self,
        institution_id: &str,
        initial_products: &[&str],
    ) -> Result<CreateSandboxPublicTokenResponse> {
        self.send_request(
            "sandbox/public_token/create",
            &CreateSandboxPublicTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                institution_id,
                initial_products,
            },
        )
        .await
    }

    /// Force a Sandbox Item into an error state.
    ///
    /// /sandbox/item/reset_login/ forces an Item into an ITEM_LOGIN_REQUIRED state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's update mode flow in the Sandbox environment. After calling /sandbox/item/reset_login, You can then use Plaid Link update mode to restore the Item to a good state. An ITEM_LOGIN_REQUIRED webhook will also be fired after a call to this endpoint, if one is associated with the Item.
    ///
    /// In the Sandbox, Items will transition to an ITEM_LOGIN_REQUIRED error state automatically after 30 days, even if this endpoint is not called.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn reset_sandbox_item(&self, access_token: &str) -> Result<ResetSandboxItemResponse> {
        self.send_request(
            "sandbox/item/reset_login",
            &ResetSandboxItemRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }

    /// Set verification status for Sandbox account.
    ///
    /// The /sandbox/item/set_verification_status endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.
    ///
    /// Note that not all Plaid developer accounts are enabled for micro-deposit based verification by default. Your account must be enabled for this feature in order to test it in Sandbox. To enable this features or check your status, contact your account manager or submit a product access Support ticket.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `account_id` - The account_id of the account whose verification status is to be modified
    /// * `verification_status` - The verification status to set the account to. Possible values: automatically_verified, verification_expired.
    pub async fn set_sandbox_verification_status(
        &self,
        access_token: &str,
        account_id: &str,
        verification_status: &str,
    ) -> Result<SetSandboxItemVerificationStatusResponse> {
        self.send_request(
            "sandbox/item/set_verification_status",
            &SetSandboxItemVerificationStatusRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                account_id,
                verification_status,
            },
        )
        .await
    }

    /// Fire a test webhook.
    ///
    /// The /sandbox/item/fire_webhook endpoint is used to test that code correctly handles webhooks. Calling this endpoint triggers a Transactions DEFAULT_UPDATE webhook to be fired for a given Sandbox Item. If the Item does not support Transactions, a SANDBOX_PRODUCT_NOT_ENABLED error will result.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `webhook_code` - The following values for webhook_code are supported: DEFAULT_UPDATE.
    pub async fn fire_webhook(
        &self,
        access_token: &str,
        webhook_code: &str,
    ) -> Result<FireWebhookResponse> {
        self.send_request(
            "sandbox/item/fire_webhook",
            &FireWebhookRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                webhook_code,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION, TEST_PRODUCTS};

    #[tokio::test]
    async fn test_create_sandbox_public_token() {
        let client = get_test_client();
        let resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, TEST_PRODUCTS)
            .await
            .unwrap();
        assert!(resp.public_token.starts_with("public-sandbox"));
    }

    #[tokio::test]
    async fn test_reset_sandbox_item() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, TEST_PRODUCTS)
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();
        let reset_resp = client
            .reset_sandbox_item(&token_resp.access_token)
            .await
            .unwrap();
        assert_eq!(reset_resp.reset_login, true);
    }
}
