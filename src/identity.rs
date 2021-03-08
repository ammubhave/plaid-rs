use serde::{Deserialize, Serialize};

use crate::accounts::AccountBalances;
use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct Identity {
    /// A list of names associated with the account by the financial institution.
    pub names: Vec<String>,
    /// A list of phone numbers associated with the account by the financial institution.
    pub phone_numbers: Vec<PhoneNumber>,
    /// A list of email addresses associated with the account by the financial institution.
    pub emails: Vec<Email>,
    /// Data about the various addresses associated with the account by the financial institution.
    pub addresses: Vec<Address>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Address {
    /// Data about the components comprising an address.
    pub data: AddressData,
    /// When true, identifies the address as the primary address on an account.
    pub primary: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AddressData {
    /// The full city name
    pub city: String,
    /// The region or state
    pub region: Option<String>,
    /// The full street address
    pub street: String,
    /// The postal code
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code
    pub country: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Email {
    /// The email address.
    pub data: String,
    /// When true, identifies the email address as the primary email on an account.
    pub primary: bool,
    /// The type of email account as described by the financial institution.
    /// Possible values: primary, secondary, other
    pub r#type: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhoneNumber {
    /// The phone number.
    pub data: String,
    /// When true, identifies the phone number as the primary number on an account.
    pub primary: Option<bool>,
    /// The type of phone number.
    /// Possible values: home, work, office, mobile, mobile1, other
    pub r#type: Option<String>,
}

#[derive(Serialize)]
struct GetIdentityRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetIdentityOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetIdentityOptions<'a> {
    /// A list of account_ids to retrieve for the Item.
    /// Note: An error will be returned if a provided account_id is not associated with the Item.
    pub account_ids: Option<&'a [&'a str]>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountWithOwners {
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
    /// Data returned by the financial institution about the account owner or owners.
    pub owners: Vec<Identity>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetIdentityResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The accounts for which Identity data has been requested
    pub accounts: Vec<AccountWithOwners>,
    /// Metadata about the Item.
    pub item: Item,
}

impl Client {
    /// Retrieve identity data.
    ///
    /// The /identity/get endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /identity/get results.
    pub async fn get_identity<'a>(
        &self,
        access_token: &str,
        options: Option<GetIdentityOptions<'a>>,
    ) -> Result<GetIdentityResponse> {
        self.send_request(
            "identity/get",
            &GetIdentityRequest {
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
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION, TEST_PRODUCTS};

    #[tokio::test]
    async fn test_get_identity() {
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
            .get_identity(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        for account in &resp.accounts {
            assert_ne!(account.owners.len(), 0);
        }
    }
}
