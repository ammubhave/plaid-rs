use crate::client::Client;
use crate::errors::{ErrorResponse, Result};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata about the Item.
#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    /// The Plaid Item ID.
    pub item_id: String,
    /// The Plaid Institution ID associated with the Item. Field is null for Items created via Same Day Micro-deposits.
    pub institution_id: Option<String>,
    /// The URL registered to receive webhooks for the Item.
    pub webhook: Option<String>,
    /// We use standard HTTP response codes for success and failure notifications, and our errors are further classified by error_type.
    pub error: Option<ErrorResponse>,
    /// A list of products available for the Item that have not yet been accessed.
    pub available_products: Vec<String>,
    /// A list of products that have been billed for the Item. Note - billed_products is populated in all environments but only requests in Production are billed.
    pub billed_products: Vec<String>,
    /// The RFC 3339 timestamp after which the consent provided by the end user will expire.
    pub consent_expiration_time: Option<DateTime<Utc>>,
    /// Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.
    pub update_type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemStatus {
    /// Information about the last successful and failed investments update for the Item.
    pub investments: Option<ProductStatus>,
    /// Information about the last successful and failed transactions update for the Item.
    pub transactions: Option<ProductStatus>,
    /// Information about the last webhook fired for the Item.
    pub last_webhook: Option<WebhookStatus>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProductStatus {
    /// ISO 8601 timestamp of the last successful transactions update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<DateTime<Utc>>,
    /// ISO 8601 timestamp of the last failed transactions update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebhookStatus {
    /// ISO 8601 timestamp of when the webhook was fired.
    pub sent_at: Option<DateTime<Utc>>,
    /// The last webhook code sent.
    pub code_sent: Option<String>,
}

#[derive(Serialize)]
struct GetItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetItemResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// Metadata about the Item.
    pub item: Item,
    /// An object with information about the status of the Item.
    pub status: Option<ItemStatus>,
    /// The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
}

#[derive(Serialize)]
struct RemoveItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RemoveItemResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}

#[derive(Serialize)]
struct UpdateItemWebhookRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    webhook: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateItemWebhookResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// Metadata about the Item.
    pub item: Item,
}

#[derive(Serialize)]
struct InvalidateAccessTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InvalidateAccessTokenResponse {
    pub request_id: String,
    pub new_access_token: String,
}

#[derive(Serialize)]
struct CreatePublicTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreatePublicTokenResponse {
    pub request_id: String,
    pub public_token: String,
}

#[derive(Serialize)]
struct ExchangePublicTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    public_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExchangePublicTokenResponse {
    pub request_id: String,
    pub access_token: String,
    pub item_id: String,
}

impl Client {
    /// Retrieve an Item.
    ///
    /// Returns information about the status of an Item.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn get_item(&self, access_token: &str) -> Result<GetItemResponse> {
        self.send_request(
            "item/get",
            &GetItemRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }

    /// Remove an Item.
    ///
    /// The /item/remove endpoint allows you to remove an Item. Once removed, the access_token associated with the Item is no longer valid and cannot be used to access any data that was associated with the Item.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn remove_item(&self, access_token: &str) -> Result<RemoveItemResponse> {
        self.send_request(
            "item/remove",
            &RemoveItemRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }

    /// Update Webhook URL.
    ///
    /// The POST /item/webhook/update allows you to update the webhook URL associated with an Item. This request triggers a WEBHOOK_UPDATE_ACKNOWLEDGED webhook to the newly specified webhook URL.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `webhook` - The new webhook URL to associate with the Item.
    pub async fn update_item_webhook(
        &self,
        access_token: &str,
        webhook: &str,
    ) -> Result<UpdateItemWebhookResponse> {
        self.send_request(
            "item/webhook/update",
            &UpdateItemWebhookRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                webhook,
            },
        )
        .await
    }

    /// Invalidate access_token.
    ///
    /// By default, the access_token associated with an Item does not expire and should be stored in a persistent, secure manner.
    ///
    /// You can use the /item/access_token/invalidate endpoint to rotate the access_token associated with an Item. The endpoint returns a new access_token and immediately invalidates the previous access_token.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn invalidate_access_token(
        &self,
        access_token: &str,
    ) -> Result<InvalidateAccessTokenResponse> {
        self.send_request(
            "item/access_token/invalidate",
            &InvalidateAccessTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }

    /// Create public token
    ///
    /// Note: As of July 2020, the /item/public_token/create endpoint is deprecated. Instead, use /link/token/create with an access_token to create a Link token for use with update mode.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn create_public_token(
        &self,
        access_token: &str,
    ) -> Result<CreatePublicTokenResponse> {
        self.send_request(
            "item/public_token/create",
            &CreatePublicTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }

    /// Exchange public token for an access token.
    ///
    /// Exchange a Link public_token for an API access_token. Link hands off the public_token client-side via the onSuccess callback once a user has successfully created an Item. The public_token is ephemeral and expires after 30 minutes.
    ///
    /// The response also includes an item_id that should be stored with the access_token. The item_id is used to identify an Item in a webhook. The item_id can also be retrieved by making an /item/get request.
    ///
    /// * `public_token` - Your public_token, obtained from the Link onSuccess callback or /sandbox/item/public_token/create.
    pub async fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> Result<ExchangePublicTokenResponse> {
        self.send_request(
            "item/public_token/exchange",
            &ExchangePublicTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                public_token,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION, TEST_PRODUCTS};

    #[tokio::test]
    async fn test_get_item() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, TEST_PRODUCTS)
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();
        let item_resp = client.get_item(&token_resp.access_token).await.unwrap();
        assert_ne!(&item_resp.item.item_id, "");
    }
}
