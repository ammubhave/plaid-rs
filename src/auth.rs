use serde::{Deserialize, Serialize};

use crate::accounts::Account;
use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Serialize)]
struct GetAuthRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetAuthOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetAuthOptions<'a> {
    /// A list of account_ids to retrieve for the Item.
    pub account_ids: Option<&'a [&'a str]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAuthResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The accounts for which numbers are being retrieved.
    pub accounts: Vec<Account>,
    /// An object containing identifying numbers used for making electronic transfers to and from the accounts.
    pub numbers: AccountNumberCollection,
    /// Metadata about the Item.
    pub item: Item,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountNumberCollection {
    /// An array of ACH numbers identifying accounts.
    pub ach: Vec<ACHNumber>,
    ///An array of EFT numbers identifying accounts.
    pub eft: Vec<EFTNumber>,
    /// An array of IBAN numbers identifying accounts.
    pub international: Vec<IBANNumber>,
    /// An array of BACS numbers identifying accounts.
    pub bacs: Vec<BACSNumber>,
}

/// An array of ACH numbers identifying accounts.
#[derive(Deserialize, Debug, Clone)]
pub struct ACHNumber {
    /// The Plaid account ID associated with the account numbers
    pub account_id: String,
    /// The ACH account number for the account
    pub account: String,
    /// The ACH routing number for the account
    pub routing: String,
    /// The wire transfer routing number for the account, if available
    pub wire_routing: Option<String>,
}

/// An array of EFT numbers identifying accounts.
#[derive(Deserialize, Debug, Clone)]
pub struct EFTNumber {
    /// The Plaid account ID associated with the account numbers
    pub account_id: String,
    /// The EFT account number for the account
    pub account: String,
    /// The EFT institution number for the account
    pub institution: String,
    /// The EFT branch number for the account
    pub branch: String,
}

/// An array of IBAN numbers identifying accounts.
#[derive(Deserialize, Debug, Clone)]
pub struct IBANNumber {
    /// The Plaid account ID associated with the account numbers
    pub account_id: String,
    /// The International Bank Account Number (IBAN) for the account
    pub iban: String,
    /// The Bank Identifier Code (BIC) for the account
    pub bic: String,
}

/// An array of BACS numbers identifying accounts.
#[derive(Deserialize, Debug, Clone)]
pub struct BACSNumber {
    /// The Plaid account ID associated with the account numbers
    pub account_id: String,
    /// The BACS account number for the account
    pub account: String,
    /// The BACS sort code for the account
    pub sort_code: String,
}

impl Client {
    /// Retrieve auth data.
    ///
    /// The /auth/get endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /auth/get results.
    pub async fn get_auth<'a>(
        &self,
        access_token: &str,
        options: Option<GetAuthOptions<'a>>,
    ) -> Result<GetAuthResponse> {
        self.send_request(
            "auth/get",
            &GetAuthRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                options,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION, TEST_PRODUCTS};

    #[tokio::test]
    async fn test_get_auth() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, TEST_PRODUCTS)
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();

        let resp = client
            .get_auth(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);

        let resp = client
            .get_auth(
                &token_resp.access_token,
                Some(GetAuthOptions {
                    account_ids: Some(&[&resp.accounts[0].account_id]),
                }),
            )
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);
    }
}
