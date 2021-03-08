use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    /// Plaid’s unique identifier for the account.
    pub account_id: String,
    /// A set of fields describing the balance for an account.
    pub balances: AccountBalances,
    /// The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    pub mask: Option<String>,
    /// The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    /// The official name of the account as given by the financial institution
    pub official_name: Option<String>,
    /// Possible values: investment, credit, depository, loan, brokerage, other
    pub r#type: String,
    /// Possible values: 401a, 401k, 403B, 457b, 529, brokerage, cash isa, education savings account, gic, health reimbursement arrangement, hsa, isa, ira, lif, lira, lrif, lrsp, non-taxable brokerage account, other, prif, rdsp, resp, rlif, rrif, pension, profit sharing plan, retirement, roth, roth 401k, rrsp, sep ira, simple ira, sipp, stock plan, thrift savings plan, tfsa, trust, ugma, utma, variable annuity, credit card, paypal, cd, checking, savings, money market, prepaid, auto, commercial, construction, consumer, home, home equity, loan, mortgage, overdraft, line of credit, student, cash management, keogh, mutual fund, recurring, rewards, safe deposit, sarsep
    pub subtype: Option<String>,
    /// The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.
    pub verification_status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountBalances {
    /// The amount of funds available to be withdrawn from the account, as determined by the financial institution.
    pub available: Option<f64>,
    /// The total amount of funds in or owed by the account.
    pub current: f64,
    /// For credit-type accounts, this represents the credit limit.
    /// For depository-type accounts, this represents the pre-arranged overdraft limit, which is common for current (checking) accounts in Europe.
    /// In North America, this field is typically only available for credit-type accounts.
    pub limit: Option<f64>,
    /// The ISO-4217 currency code of the balance. Always null if unofficial_currency_code is non-null.
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the balance. Always null if iso_currency_code is non-null.
    pub unofficial_currency_code: Option<String>,
}

#[derive(Serialize)]
struct GetBalancesRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetBalancesOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetBalancesOptions<'a> {
    /// A list of account_ids to retrieve for the Item.
    pub account_ids: Option<&'a [&'a str]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetBalancesResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// An array of financial institution accounts associated with the Item.
    pub accounts: Vec<Account>,
}

#[derive(Serialize)]
struct GetAccountsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetAccountsOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetAccountsOptions<'a> {
    /// An array of account_ids to retrieve for the Account.
    pub account_ids: Option<&'a [&'a str]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// An array of financial institution accounts associated with the Item.
    pub accounts: Vec<Account>,
    /// Metadata about the Item.
    pub item: Item,
}

impl Client {
    /// Retrieve real-time balance data.
    ///
    /// The /accounts/balance/get endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only /accounts/balance/get forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, balance itself is not a product that can be used to initialize Link.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /accounts/balance/get results.
    pub async fn get_balances<'a>(
        &self,
        access_token: &str,
        options: Option<GetBalancesOptions<'a>>,
    ) -> Result<GetBalancesResponse> {
        self.send_request(
            "accounts/balance/get",
            &GetBalancesRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                options,
            },
        )
        .await
    }

    /// Retrieve accounts.
    ///
    /// The /accounts/get endpoint can be used to retrieve information for any linked Item. Note that some information is nullable. Plaid will only return active bank accounts, i.e. accounts that are not closed and are capable of carrying a balance.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /accounts/get results.
    pub async fn get_accounts<'a>(
        &self,
        access_token: &str,
        options: Option<GetAccountsOptions<'a>>,
    ) -> Result<GetAccountsResponse> {
        self.send_request(
            "accounts/get",
            &GetAccountsRequest {
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
    async fn test_get_accounts() {
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
            .get_accounts(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);

        let resp = client
            .get_accounts(
                &token_resp.access_token,
                Some(GetAccountsOptions {
                    account_ids: Some(&[&resp.accounts[0].account_id]),
                }),
            )
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_eq!(resp.item.item_id.is_empty(), false);
    }

    #[tokio::test]
    async fn test_get_balances() {
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
            .get_balances(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);

        let resp = client
            .get_balances(
                &token_resp.access_token,
                Some(GetBalancesOptions {
                    account_ids: Some(&[&resp.accounts[0].account_id]),
                }),
            )
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
    }
}
