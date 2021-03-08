use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

/// A JSON Web Key (JWK) that can be used in conjunction with JWT libraries to verify Plaid webhooks
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookVerificationKey {
    /// The alg member identifies the cryptographic algorithm family used with the key.
    pub alg: String,
    /// The crv member identifies the cryptographic curve used with the key.
    pub crv: String,
    /// The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    pub kid: String,
    /// The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    pub kty: String,
    /// The use (public key use) parameter identifies the intended use of the public key.
    pub r#use: String,
    /// The x member contains the x coordinate for the elliptic curve point.
    pub x: String,
    /// The y member contains the y coordinate for the elliptic curve point.
    pub y: String,
    pub created_at: i64,
    pub expired_at: Option<i64>,
}

#[derive(Serialize)]
struct GetWebhookVerificationKeyRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    key_id: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetWebhookVerificationKeyResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// A JSON Web Key (JWK) that can be used in conjunction with JWT libraries to verify Plaid webhooks
    pub key: WebhookVerificationKey,
}

impl Client {
    /// Get webhook verification key.
    ///
    /// Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the Plaid-Verification header.
    ///
    /// The /webhook_verification_key/get endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.
    ///
    /// * `key_id` - The key ID ( kid ) from the JWT header.
    pub async fn get_webhook_verification_key(
        &self,
        key_id: &str,
    ) -> Result<GetWebhookVerificationKeyResponse> {
        self.send_request(
            "webhook_verification_key/get",
            &GetWebhookVerificationKeyRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                key_id,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::tests::get_test_client;

    #[tokio::test]
    async fn test_get_webhook_verification_key() {
        let client = get_test_client();
        let resp = client
            .get_webhook_verification_key("6c5516e1-92dc-479e-a8ff-5a51992e0001")
            .await
            .unwrap();
        assert!(!resp.key.alg.is_empty());
        assert!(!resp.key.crv.is_empty());
        assert!(!resp.key.kid.is_empty());
        assert!(!resp.key.kty.is_empty());
        assert!(!resp.key.r#use.is_empty());
        assert!(!resp.key.x.is_empty());
        assert!(!resp.key.y.is_empty());
        assert_ne!(!resp.key.created_at, 0);
    }
}
