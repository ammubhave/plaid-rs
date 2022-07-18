use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::accounts::Account;
use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct Transaction {
    /// The unique ID of the transaction. Like all Plaid identifiers, the transaction_id is case sensitive.
    pub transaction_id: String,
    /// The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    pub account_owner: Option<String>,
    /// The ID of a posted transaction's associated pending transaction, where applicable.
    pub pending_transaction_id: Option<String>,
    /// When true, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    pub pending: bool,
    /// The channel used to make a payment.
    /// Possible values: online, in store, other
    pub payment_channel: String,
    /// Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be null.
    pub payment_meta: PaymentMeta,
    /// The merchant name or transaction description.
    pub name: String,
    /// The merchant name, as extracted by Plaid from the name field.
    pub merchant_name: Option<String>,
    /// A representation of where a transaction took place
    pub location: Location,
    /// The date that the transaction was authorized. Dates are returned in an ISO 8601 format ( YYYY-MM-DD ).
    pub authorized_date: Option<NaiveDate>,
    /// Date and time when a transaction was authorized in ISO 8601 format ( YYYY-MM-DDTHH:mm:ssZ ).
    pub authorized_datetime: Option<DateTime<Utc>>,
    /// For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an ISO 8601 format ( YYYY-MM-DD ).
    pub date: NaiveDate,
    /// Date and time when a transaction was posted in ISO 8601 format ( YYYY-MM-DDTHH:mm:ssZ ).
    pub datetime: Option<DateTime<Utc>>,
    /// The ID of the category to which this transaction belongs.
    pub category_id: String,
    /// A hierarchical array of the categories to which this transaction belongs
    pub category: Option<Vec<String>>,
    /// The unofficial currency code associated with the transaction.
    pub unofficial_currency_code: Option<String>,
    /// The ISO-4217 currency code of the transaction.
    pub iso_currency_code: Option<String>,
    /// The settled value of the transaction, denominated in the account's currency, as stated in iso_currency_code or unofficial_currency_code. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    /// The ID of the account in which this transaction occurred.
    pub account_id: String,
    /// An identifier classifying the transaction type.
    /// This field is only populated for European institutions. For institutions in the US and Canada, this field is set to null.
    /// Possible values: adjustment, atm, bank charge, bill payment, cash, cashback, cheque, direct debit, interest, purchase, standing order, transfer, null
    pub transaction_code: Option<String>,
}

/// Transaction information specific to inter-bank transfers.
#[derive(Deserialize, Debug, Clone)]
pub struct PaymentMeta {
    /// The transaction reference number supplied by the financial institution.
    pub reference_number: Option<String>,
    /// The ACH PPD ID for the payer.
    pub ppd_id: Option<String>,
    /// For transfers, the party that is receiving the transaction.
    pub payee: Option<String>,
    /// The party initiating a wire transfer. Will be null if the transaction is not a wire transfer.
    pub by_order_of: Option<String>,
    /// For transfers, the party that is paying the transaction.
    pub payer: Option<String>,
    /// The type of transfer, e.g. 'ACH'
    pub payment_method: Option<String>,
    /// The name of the payment processor
    pub payment_processor: Option<String>,
    /// The payer-supplied description of the transfer.
    pub reason: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Location {
    /// The street address where the transaction occurred.
    pub address: Option<String>,
    /// The city where the transaction occurred.
    pub city: Option<String>,
    /// The region or state where the transaction occurred.
    pub region: Option<String>,
    /// The postal code where the transaction occurred.
    pub postal_code: Option<String>,
    /// The ISO 3166-1 alpha-2 country code where the transaction occurred.
    pub country: Option<String>,
    /// The latitude where the transaction occurred.
    pub lat: Option<f64>,
    /// The longitude where the transaction occurred.
    pub lon: Option<f64>,
    /// The merchant defined store number where the transaction occurred.
    pub store_number: Option<String>,
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
struct GetTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetTransactionsOptions<'a>>,
}

#[derive(Serialize)]
struct SyncTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    cursor: Option<String>,
    count: u8
    // #[serde(skip_serializing_if = "Option::is_none")]
    // options: Option<GetTransactionsOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetTransactionsOptions<'a> {
    /// A list of account_ids to retrieve for the Item
    pub account_ids: Option<&'a [&'a str]>,
    /// The number of transactions to fetch.
    pub count: i32,
    /// The number of transactions to skip. The default value is 0.
    pub offset: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SyncTransactionsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    pub next_cursor: String,
    pub has_more: bool,
    /// An array containing the added transactions
    pub added: Vec<Transaction>,
    // modified
    // removed
}


#[derive(Deserialize, Debug, Clone)]
pub struct GetTransactionsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// An array containing the accounts associated with the Item for which transactions are being returned. Each transaction can be mapped to its corresponding account via the account_id field.
    pub accounts: Vec<Account>,
    /// An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the count parameter.
    pub transactions: Vec<Transaction>,
    /// The total number of transactions available within the date range specified. If total_transactions is larger than the size of the transactions array, more transactions are available and can be fetched via manipulating the offset parameter.
    pub total_transactions: i32,
    /// Metadata about the Item.
    pub item: Item,
}

#[derive(Serialize)]
struct RefreshTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RefreshTransactionsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}

impl Client {
    /// Sync transaction data.
    ///
    /// The /transactions/sync handler
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `start_date` - The earliest date for which data should be returned.
    /// * `end_date` - The latest date for which data should be returned.
    /// * `options` - An optional object to be used with the request.
    pub async fn sync_transactions<'a>(
        &self,
        access_token: &str,
        cursor: Option<String>,
        count: u8,
        // options: Option<GetTransactionsOptions<'a>>,
    ) -> Result<SyncTransactionsResponse> {
        self.send_request(
            "transactions/sync",
            &SyncTransactionsRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
                cursor,
                count
            },
        )
        .await
    }

    /// Get transaction data.
    ///
    /// The /transactions/get endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (the list of loan-type accounts supported is the same as for Liabilities; for details, see the /liabilities/get endpoint). For transaction history from investments accounts, use the Investments endpoint instead. Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.
    ///
    /// Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift. Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in /transactions/get. For more details, see Pending and posted transactions.
    ///
    /// Due to the potentially large number of transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the total_transactions response body field to fetch all available transactions.
    ///
    /// Note that data may not be immediately available to /transactions/get. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with transactions, or upon the first call to /transactions/get, if it wasn't. To be alerted when transaction data is ready to be fetched, listen for the INITIAL_UPDATE and HISTORICAL_UPDATE webhooks. If no transaction history is ready when /transactions/get is called, it will return a PRODUCT_NOT_READY error.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `start_date` - The earliest date for which data should be returned.
    /// * `end_date` - The latest date for which data should be returned.
    /// * `options` - An optional object to be used with the request.
    pub async fn get_transactions<'a>(
        &self,
        access_token: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        options: Option<GetTransactionsOptions<'a>>,
    ) -> Result<GetTransactionsResponse> {
        self.send_request(
            "transactions/get",
            &GetTransactionsRequest {
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

    /// Refresh transaction data.
    ///
    /// /transactions/refresh is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling /transactions/refresh, Plaid will fire a webhook: TRANSACTIONS_REMOVED will be fired if any removed transactions are detected, and DEFAULT_UPDATE will be fired if any new transactions are detected. New transactions can be fetched by calling /transactions/get.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    pub async fn refresh_transactions(
        &self,
        access_token: &str,
    ) -> Result<RefreshTransactionsResponse> {
        self.send_request(
            "transactions/refresh",
            &RefreshTransactionsRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                access_token,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use super::*;
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION, TEST_PRODUCTS};
    use crate::errors::Error;

    #[tokio::test]
    async fn test_get_transactions() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, TEST_PRODUCTS)
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();
        let end_date = Utc::now().naive_utc().date();
        let start_date = end_date.sub(chrono::Duration::days(365));

        let mut resp = client
            .get_transactions(&token_resp.access_token, start_date, end_date, None)
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
                .get_transactions(&token_resp.access_token, start_date, end_date, None)
                .await;
        }
        let resp = resp.unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_ne!(resp.transactions.len(), 0);

        let mut resp = client
            .get_transactions(
                &token_resp.access_token,
                start_date,
                end_date,
                Some(GetTransactionsOptions {
                    account_ids: None,
                    count: 2,
                    offset: 1,
                }),
            )
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
                .get_transactions(
                    &token_resp.access_token,
                    start_date,
                    end_date,
                    Some(GetTransactionsOptions {
                        account_ids: None,
                        count: 2,
                        offset: 1,
                    }),
                )
                .await;
        }
        let resp = resp.unwrap();
        assert_ne!(resp.transactions.len(), 0);
    }

    #[tokio::test]
    async fn test_refresh_transactions() {
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
            .refresh_transactions(&token_resp.access_token)
            .await
            .unwrap();
        assert_ne!(&resp.request_id, "");
    }
}
