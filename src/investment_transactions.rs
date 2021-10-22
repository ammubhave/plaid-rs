use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::accounts::Account;
use crate::client::Client;
use crate::errors::Result;
use crate::holdings::Security;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct InvestmentTransaction {
    /// The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the investment_transaction_id is case sensitive.
    pub investment_transaction_id: String,
    pub cancel_transaction_id: Option<String>,
    /// The account_id of the account against which this transaction posted.
    pub account_id: String,
    /// The security_id to which this transaction is related.
    pub security_id: Option<String>,
    /// The ISO-8601 posting date for the transaction, or transacted date for pending transactions.
    pub date: NaiveDate,
    /// The institution’s description of the transaction.
    pub name: String,
    /// The number of units of the security involved in this transactions
    pub quantity: f64,
    /// The complete value of the transaction.
    pub amount: f64,
    /// The price of the security at which this transaction occurred.
    pub price: f64,
    /// The combined value of all fees applied to this transaction
    pub fees: Option<f64>,
    /// Possible values: buy, sell, cancel, cash, fee, transfer
    pub r#type: String,
    /// transaction subtype
    pub subtype: String,
    /// The ISO-4217 currency code of the transaction. Always null if unofficial_currency_code is non-null.
    pub iso_currency_code: Option<String>,
    /// The unofficial currency code associated with the holding.
    pub unofficial_currency_code: Option<String>,
}

#[derive(Serialize)]
struct GetInvestmentTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInvestmentTransactionsOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetInvestmentTransactionsOptions<'a> {
    /// A list of account_ids to retrieve for the Item
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<&'a [&'a str]>,
    /// The number of transactions to fetch. Maximum: 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// The number of transactions to skip when fetching transaction history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl Default for GetInvestmentTransactionsOptions<'_> {
    fn default() -> Self {
        Self {
            account_ids: None,
            count: None,
            offset: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetInvestmentTransactionsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The accounts for which transaction history is being fetched.
    pub accounts: Vec<Account>,
    /// All securities for which there is a corresponding transaction being fetched.
    pub securities: Vec<Security>,
    /// The transactions being fetched
    pub investment_transactions: Vec<InvestmentTransaction>,
    /// The total number of transactions available within the date range specified.
    pub total_investment_transactions: i32,
    /// Metadata about the Item.
    pub item: Item,
}

impl Client {
    /// Get investment transactions.
    ///
    /// The /investments/transactions/get endpoint allows developers to retrieve user-authorized transaction data for investment accounts.
    ///
    /// Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.
    ///
    /// Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the total_investment_transactions response body field to fetch all available investment transactions.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `start_date` - The earliest date for which to fetch transaction history.
    /// * `end_date` - The most recent date for which to fetch transaction history.
    /// * `options` - An optional object to filter /investments/transactions/get results.
    pub async fn get_investment_transactions<'a>(
        &self,
        access_token: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        options: Option<GetInvestmentTransactionsOptions<'a>>,
    ) -> Result<GetInvestmentTransactionsResponse> {
        self.send_request(
            "investments/transactions/get",
            &GetInvestmentTransactionsRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                start_date,
                end_date,
                options,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use std::ops::Sub;

    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION};
    use crate::errors::Error;

    // The following test fails because plaid responds back with NOT_FOUND. Needs debugging.
    #[tokio::test]
    async fn test_get_investment_transactions() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, &["investments"])
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();
        let end_date = Utc::now().naive_utc().date();
        let start_date = end_date.sub(chrono::Duration::days(3650));
        let mut resp = client
            .get_investment_transactions(&token_resp.access_token, start_date, end_date, None)
            .await;
        while resp.is_err() {
            let err = resp.unwrap_err();
            if let Error::Plaid(err) = err {
                assert_eq!(err.error_code, "PRODUCT_NOT_READY");
            } else {
                assert!(false);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            resp = client
                .get_investment_transactions(&token_resp.access_token, start_date, end_date, None)
                .await;
        }
        let resp = resp.unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_ne!(resp.investment_transactions.len(), 0);
        for investment_transaction in &resp.investment_transactions {
            assert_ne!(investment_transaction.subtype.len(), 0);
        }
        assert_ne!(resp.securities.len(), 0);
    }
}
