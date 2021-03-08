use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::Client;
use crate::errors::Result;

#[derive(Serialize, Debug, Clone)]
pub struct LinkTokenUser<'a> {
    /// A unique ID representing the end user.
    pub client_user_id: &'a str,
    /// The user's full legal name. This is an optional field used in the returning user experience to associate Items to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<&'a str>,
    /// The user's phone number in E.164 format. This field is optional, but required to enable the returning user experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<&'a str>,
    /// The date and time the phone number was verified in ISO 8601 format (YYYY-MM-DDThh:mm:ssZ). This field is optional, but required to enable any returning user experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_verified_time: Option<DateTime<Utc>>,
    /// The user's email address. This field is optional, but required to enable the pre-authenticated returning user flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<&'a str>,
    /// The date and time the email address was verified in ISO 8601 format (YYYY-MM-DDThh:mm:ssZ). This is an optional field used in the returning user experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address_verified_time: Option<DateTime<Utc>>,
    /// To be provided in the format "ddd-dd-dddd". This field is optional and will support not-yet-implemented functionality for new products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<&'a str>,
    /// To be provided in the format "yyyy-mm-dd". This field is optional and will support not-yet-implemented functionality for new products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<&'a str>,
}

impl Default for LinkTokenUser<'_> {
    fn default() -> Self {
        Self {
            client_user_id: "",
            legal_name: None,
            phone_number: None,
            phone_number_verified_time: None,
            email_address: None,
            email_address_verified_time: None,
            ssn: None,
            date_of_birth: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinkTokenConfigs<'a> {
    /// An object specifying information about the end user who will be linking their account.
    pub user: LinkTokenUser<'a>,
    /// The name of your application, as it should be displayed in Link.
    pub client_name: &'a str,
    /// The language that Link should be displayed in.
    pub language: &'a str,
    /// Specify an array of Plaid-supported country codes using the ISO-3166-1 alpha-2 country code standard.
    pub country_codes: &'a [&'a str],
    /// List of Plaid product(s) you wish to use.
    pub products: Option<&'a [&'a str]>,
    /// The destination URL to which any webhooks should be sent.
    pub webhook: Option<&'a str>,
    /// The name of the Link customization from the Plaid Dashboard to be applied to Link.
    pub link_customization_name: Option<&'a str>,
    pub account_filters: Option<HashMap<&'a str, HashMap<&'a str, Vec<&'a str>>>>,
    /// A URI indicating the destination where a user should be forwarded after completing the Link flow
    pub redirect_uri: Option<&'a str>,
    /// The name of your app's Android package.
    pub android_package_name: Option<&'a str>,
}

#[derive(Serialize)]
struct CreateLinkTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    client_name: &'a str,
    language: &'a str,
    country_codes: &'a [&'a str],
    user: LinkTokenUser<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    webhook: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link_customization_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account_filters: Option<HashMap<&'a str, HashMap<&'a str, Vec<&'a str>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_uri: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    android_package_name: Option<&'a str>,
}

impl Default for LinkTokenConfigs<'_> {
    fn default() -> Self {
        Self {
            user: Default::default(),
            client_name: "",
            language: "en",
            country_codes: &["US"],
            products: None,
            webhook: None,
            link_customization_name: None,
            account_filters: None,
            redirect_uri: None,
            android_package_name: None,
        }
    }
}

#[derive(Serialize)]
struct GetLinkTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    link_token: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct CreateLinkTokenResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// A link_token, which can be supplied to Link in order to initialize it and receive a public_token, which can be exchanged for an access_token.
    pub link_token: String,
    /// The expiration date for the link_token, in ISO 8601 format.
    pub expiration: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct GetLinkTokenMetadataResponse {
    /// The products specified in the /link/token/create call.
    pub initial_products: Vec<String>,
    /// The webhook specified in the /link/token/create call.
    pub webhook: Option<String>,
    /// The country_codes specified in the /link/token/create call.
    pub country_codes: Vec<String>,
    /// The language specified in the /link/token/create call.
    pub language: Option<String>,
    /// The account_filters specified in the original call to /link/token/create.
    pub account_filters: HashMap<String, HashMap<String, Vec<String>>>,
    /// The redirect_uri specified in the /link/token/create call.
    pub redirect_uri: Option<String>,
    /// The client_name specified in the /link/token/create call.
    pub client_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GetLinkTokenResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// A link_token, which can be supplied to Link in order to initialize it and receive a public_token, which can be exchanged for an access_token.
    pub link_token: String,
    /// The creation timestamp for the link_token, in ISO 8601 format.
    pub created_at: Option<DateTime<Utc>>,
    /// The expiration timestamp for the link_token, in ISO 8601 format.
    pub expiration: Option<DateTime<Utc>>,
    /// An object specifying the arguments originally provided to the /link/token/create call.
    pub metadata: GetLinkTokenMetadataResponse,
}

impl Client {
    /// Create Link Token.
    ///
    /// The /link/token/create endpoint creates a link_token, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a public_token, which can then be exchanged for an access_token via /item/public_token/exchange as part of the main Link flow.
    ///
    /// A link_token generated by /link/token/create is also used to initialize other Link flows, such as the update mode flow for tokens with expired credentials, or the Payment Initiation (Europe) flow.
    ///
    /// * `configs` - Parameters to use for creating link token.
    pub async fn create_link_token<'a>(
        &self,
        configs: LinkTokenConfigs<'a>,
    ) -> Result<CreateLinkTokenResponse> {
        self.send_request(
            "link/token/create",
            &CreateLinkTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                client_name: configs.client_name,
                language: configs.language,
                country_codes: configs.country_codes,
                user: configs.user,
                products: configs.products,
                webhook: configs.webhook,
                link_customization_name: configs.link_customization_name,
                account_filters: configs.account_filters,
                redirect_uri: configs.redirect_uri,
                android_package_name: configs.android_package_name,
            },
        )
        .await
    }

    /// Get Link Token.
    ///
    /// The /link/token/get endpoint gets information about a previously-created link_token using the /link/token/create endpoint. It can be useful for debugging purposes.
    ///
    /// * `link_token` - A link_token from a previous invocation of /link/token/create
    pub async fn get_link_token(&self, link_token: &str) -> Result<GetLinkTokenResponse> {
        self.send_request(
            "link/token/get",
            &GetLinkTokenRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                link_token,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::tests::get_test_client;

    use super::*;

    #[tokio::test]
    async fn test_create_link_token_required() {
        let client = get_test_client();

        let time_now = Utc::now().to_rfc3339();
        let resp = client
            .create_link_token(LinkTokenConfigs {
                user: LinkTokenUser {
                    client_user_id: &time_now,
                    ..Default::default()
                },
                client_name: "Plaid Test",
                products: Some(&["auth"]),
                country_codes: &["US"],
                language: "en",
                ..Default::default()
            })
            .await
            .unwrap();
        assert!(resp.link_token.starts_with("link-sandbox"));
        assert_ne!(resp.expiration.timestamp(), 0);
    }

    #[tokio::test]
    async fn test_create_link_token_optional() {
        let client = get_test_client();

        let time_now = Utc::now().to_rfc3339();
        let resp = client
            .create_link_token(LinkTokenConfigs {
                user: LinkTokenUser {
                    client_user_id: &time_now,
                    legal_name: Some("Legal Name"),
                    phone_number: Some("2025550165"),
                    email_address: Some("test@email.com"),
                    phone_number_verified_time: Some(Utc::now()),
                    email_address_verified_time: Some(Utc::now()),
                    ssn: None,
                    date_of_birth: None,
                },
                client_name: "Plaid Test",
                products: Some(&["auth"]),
                country_codes: &["US"],
                language: "en",
                webhook: Some("https://webhook-uri.com"),
                link_customization_name: Some("default"),
                account_filters: Some(
                    vec![(
                        "depository",
                        vec![("account_subtypes", vec!["checking", "savings"])]
                            .into_iter()
                            .collect(),
                    )]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            })
            .await
            .unwrap();
        assert!(resp.link_token.starts_with("link-sandbox"));
        assert_ne!(resp.expiration.timestamp(), 0);
    }

    #[tokio::test]
    async fn test_create_link_token_then_get() {
        let client = get_test_client();

        let time_now = Utc::now().to_rfc3339();
        let create_resp = client
            .create_link_token(LinkTokenConfigs {
                user: LinkTokenUser {
                    client_user_id: &time_now,
                    legal_name: Some("Legal Name"),
                    phone_number: Some("2025550165"),
                    email_address: Some("test@email.com"),
                    phone_number_verified_time: Some(Utc::now()),
                    email_address_verified_time: Some(Utc::now()),
                    ssn: None,
                    date_of_birth: None,
                },
                client_name: "Plaid Test",
                products: Some(&["auth"]),
                country_codes: &["US"],
                language: "en",
                webhook: Some("https://webhook-uri.com"),
                link_customization_name: Some("default"),
                account_filters: Some(
                    vec![(
                        "depository",
                        vec![("account_subtypes", vec!["checking", "savings"])]
                            .into_iter()
                            .collect(),
                    )]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            })
            .await
            .unwrap();
        assert!(create_resp.link_token.starts_with("link-sandbox"));
        assert_ne!(create_resp.expiration.timestamp(), 0);

        let get_resp = client
            .get_link_token(&create_resp.link_token)
            .await
            .unwrap();
        assert_eq!(create_resp.link_token, get_resp.link_token);
        assert_eq!(get_resp.metadata.initial_products, &["auth"]);
        assert_eq!(
            get_resp.metadata.webhook,
            Some("https://webhook-uri.com".to_string())
        );
        assert_eq!(get_resp.metadata.country_codes, &["US"]);
        assert_eq!(get_resp.metadata.language, Some("en".to_string()));
        assert_eq!(get_resp.metadata.account_filters.len(), 1);
        assert_eq!(
            get_resp.metadata.client_name,
            Some("Plaid Test".to_string())
        );
        assert_ne!(get_resp.expiration.unwrap().timestamp(), 0);
        assert_ne!(get_resp.created_at.unwrap().timestamp(), 0);
    }
}
