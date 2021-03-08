use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::accounts::Account;
use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct Security {
    /// A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the security_id is case sensitive.
    pub security_id: String,
    /// 12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    /// 9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    /// 7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    /// An identifier given to the security by the institution
    pub institution_security_id: Option<String>,
    /// If institution_security_id is present, this field indicates the Plaid institution_id of the institution to whom the identifier belongs.
    pub institution_id: Option<String>,
    /// In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security.
    pub proxy_security_id: Option<String>,
    /// A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    /// The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
    /// Indicates that a security is a highly liquid asset and can be treated like cash.
    pub is_cash_equivalent: bool,
    /// The security type of the holding.
    pub r#type: Option<String>,
    /// Price of the security at the close of the previous trading session. null for non-public securities.
    pub close_price: Option<f64>,
    /// Date for which close_price is accurate. Always null if close_price is null.
    pub close_price_as_of: Option<NaiveDate>,
    /// The ISO-4217 currency code of the price given. Always null if unofficial_currency_code is non-null.
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the security.
    pub unofficial_currency_code: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Holding {
    /// The Plaid account_id associated with the holding.
    pub account_id: String,
    /// The Plaid security_id associated with the holding.
    pub security_id: String,
    /// The last price given by the institution for this security.
    pub institution_price: f64,
    /// The date at which institution_price was current.
    pub institution_price_as_of: Option<NaiveDate>,
    /// The value of the holding, as reported by the institution.
    pub institution_value: f64,
    /// The cost basis of the holding.
    pub cost_basis: Option<f64>,
    /// The total quantity of the asset held, as reported by the financial institution.
    pub quantity: f64,
    /// The ISO-4217 currency code of the holding. Always null if unofficial_currency_code is non-null.
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding.
    pub unofficial_currency_code: Option<String>,
}

#[derive(Serialize)]
struct GetHoldingsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetHoldingsOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetHoldingsOptions<'a> {
    /// A list of account_ids to retrieve for the Item.
    pub account_ids: Option<&'a [&'a str]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetHoldingsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The accounts associated with the Item
    pub accounts: Vec<Account>,
    /// The holdings belonging to investment accounts associated with the Item.
    pub holdings: Vec<Holding>,
    // /// Objects describing the securities held in the accounts associated with the Item.
    pub securities: Vec<Security>,
    /// Metadata about the Item.
    pub item: Item,
}

impl Client {
    /// Get Investment holdings.
    ///
    /// The /investments/holdings/get endpoint allows developers to receive user-authorized stock position data for investment-type accounts.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /investments/holdings/get results.
    pub async fn get_holdings<'a>(
        &self,
        access_token: &str,
        options: Option<GetHoldingsOptions<'a>>,
    ) -> Result<GetHoldingsResponse> {
        self.send_request(
            "investments/holdings/get",
            &GetHoldingsRequest {
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
    async fn test_get_holdings() {
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
            .get_holdings(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_ne!(resp.securities.len(), 0);
        assert_ne!(resp.holdings.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);

        let resp = client
            .get_holdings(
                &token_resp.access_token,
                Some(GetHoldingsOptions {
                    account_ids: Some(&[&resp.holdings[0].account_id]),
                }),
            )
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_ne!(resp.holdings.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);
    }
}
