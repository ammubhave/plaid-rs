use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::accounts::Account;
use crate::client::Client;
use crate::errors::Result;
use crate::item::Item;

#[derive(Deserialize, Debug, Clone)]
pub struct CreditLiability {
    /// The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    /// The various interest rates that apply to the account.
    pub aprs: Vec<APR>,
    /// true if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    /// The amount of the last payment.
    pub last_payment_amount: f64,
    /// The date of the last payment. Dates are returned in an ISO 8601 format (YYYY-MM-DD). Availability for this field is limited.
    pub last_payment_date: Option<NaiveDate>,
    /// The outstanding balance on the last statement. Availability for this field is limited.
    pub last_statement_balance: f64,
    /// The date of the last statement. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub last_statement_issue_date: NaiveDate,
    /// The minimum payment due for the next billing cycle.
    pub minimum_payment_amount: f64,
    /// The due date for the next payment. The due date is null if a payment is not expected. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub next_payment_due_date: Option<NaiveDate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct APR {
    /// Annual Percentage Rate applied.
    pub apr_percentage: f64,
    /// The type of balance to which the APR applies.
    /// Possible values: balance_transfer_apr, cash_apr, purchase_apr, special
    pub apr_type: String,
    /// Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance.
    pub balance_subject_to_api: Option<f64>,
    /// Amount of money charged due to interest from last statement.
    pub interest_charge_amount: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MortgageLiability {
    /// The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    /// The account number of the loan.
    pub account_number: String,
    /// The current outstanding amount charged for late payment.
    pub current_late_fee: Option<f64>,
    /// Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    pub escrow_balance: Option<f64>,
    /// Indicates whether the borrower has private mortgage insurance in effect.
    pub has_pmi: Option<bool>,
    /// Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    pub has_prepayment_penalty: Option<bool>,
    /// Object containing metadata about the interest rate for the mortgage.
    pub interest_rate: MortgageInterestRate,
    /// The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    /// The date of the last payment. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub last_payment_date: Option<NaiveDate>,
    /// Description of the type of loan, for example conventional, fixed, or variable. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    pub loan_type_descrption: Option<String>,
    /// Full duration of mortgage as at origination (e.g. 10 year).
    pub loan_term: Option<String>,
    /// Original date on which mortgage is due in full. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub maturity_date: Option<NaiveDate>,
    /// The amount of the next payment.
    pub next_monthly_payment: Option<f64>,
    /// The due date for the next payment. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub next_payment_due_date: Option<NaiveDate>,
    /// The date on which the loan was initially lent. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub origination_date: Option<NaiveDate>,
    /// The original principal balance of the mortgage.
    pub origination_principal_amount: Option<f64>,
    /// Amount of loan (principal + interest) past due for payment.
    pub past_due_amount: Option<f64>,
    /// Object containing fields describing property address.
    pub property_address: MortgagePropertyAddress,
    /// The year to date (YTD) interest paid.
    pub ytd_interest_paid: Option<f64>,
    /// The YTD principal paid.
    pub ytd_principal_paid: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MortgageInterestRate {
    /// Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    pub percentage: Option<f64>,
    /// The type of interest charged (fixed or variable).
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MortgagePropertyAddress {
    /// The city name.
    pub city: Option<String>,
    /// The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    /// The five or nine digit postal code.
    pub postal_code: Option<String>,
    /// The region or state (example "NC").
    pub region: Option<String>,
    /// The full street address (example "564 Main Street, Apt 15").
    pub street: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StudentLoanLiability {
    /// The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    /// The account number of the loan.
    pub account_number: Option<String>,
    /// The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub disbursement_dates: Option<Vec<NaiveDate>>,
    /// The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub expected_payoff_date: Option<String>,
    /// The guarantor of the student loan.
    pub guarantor: Option<String>,
    /// The interest rate on the loan as a percentage.
    pub interest_rate_percentage: f64,
    /// true if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    /// The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    /// The date of the last payment. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub last_payment_date: Option<NaiveDate>,
    /// The outstanding balance on the last statement. This field could also be interpreted as the next payment due. Availability for this field is limited.
    pub last_statement_balance: Option<f64>,
    /// The date of the last statement. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<NaiveDate>,
    /// The type of loan, e.g., "Consolidation Loans".
    pub loan_name: Option<String>,
    /// An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    /// The minimum payment due for the next billing cycle.
    pub minimum_payment_amount: Option<f64>,
    /// The due date for the next payment.
    pub next_payment_due_date: Option<NaiveDate>,
    /// The date on which the loan was initially lent. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub origination_date: Option<NaiveDate>,
    /// The original principal balance of the loan.
    pub origination_principal_amount: Option<f64>,
    /// The total dollar amount of the accrued interest balance. For Sallie Mae ( ins_116944), this amount is included in the current balance of the loan, so this field will return as null.
    pub origination_interest_amount: Option<f64>,
    /// The relevant account number that should be used to reference this loan for payments. In the majority of cases, payment_reference_number will match account_number, but in some institutions, such as Great Lakes (ins_116861), it will be different.
    pub payment_reference_number: Option<String>,
    /// Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (ins_116527).
    pub pslf_status: PSLFStatus,
    /// An object representing the repayment plan for the student loan
    pub repayment_plan: StudentLoanRepaymentPlan,
    /// The sequence number of the student loan. Heartland ECSI (ins_116948) does not make this field available.
    pub sequence_number: Option<String>,
    /// The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub servicer_address: StudentLoanServicerAddress,
    /// The year to date (YTD) interest paid. Availability for this field is limited.
    pub ytd_interest_paid: Option<f64>,
    /// The year to date (YTD) principal paid. Availability for this field is limited.
    pub ytd_principal_paid: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StudentLoanStatus {
    /// The date until which the loan will be in its current status. Dates are returned in an ISO 8601 format (YYYY-MM-DD).
    pub end_date: Option<NaiveDate>,
    /// The status type of the student loan
    /// Possible values: cancelled, charged off, claim, consolidated, deferment, delinquent, discharged, extension, forbearance, in grace, in military, in school, not fully disbursed, other, paid in full, refunded, repayment, transferred
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PSLFStatus {
    /// The estimated date borrower will have completed 120 qualifying monthly payments. Returned in ISO 8601 format (YYYY-MM-DD).
    pub estimated_eligibility_date: Option<NaiveDate>,
    /// The number of qualifying payments that have been made.
    pub payments_made: Option<i64>,
    /// The number of qualifying payments remaining.
    pub payments_remaining: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StudentLoanRepaymentPlan {
    /// The description of the repayment plan as provided by the servicer.
    pub description: Option<String>,
    /// The type of the repayment plan.
    /// Possible values: extended graduated, extended standard, graduated, income-contingent repayment, income-based repayment, interest-only, other, pay as you earn, revised pay as you earn, standard
    pub r#type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StudentLoanServicerAddress {
    /// The full city name
    pub city: Option<String>,
    /// The region or state
    /// Example: "NC"
    pub region: Option<String>,
    /// The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    /// The postal code
    pub postal_code: Option<String>,
    /// The full street address
    /// Example: "564 Main Street, APT 15"
    pub street: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Liabilities {
    /// The credit accounts returned. If no credit accounts are returned, credit will not be present in the schema.
    pub credit: Option<Vec<CreditLiability>>,
    /// The mortgage accounts returned. If no mortgage accounts are returned, mortgage will not be present in the schema.
    pub mortgage: Option<Vec<MortgageLiability>>,
    /// The student loan accounts returned. If no student loan accounts are returned, student will not be present in the schema.
    pub student: Option<Vec<StudentLoanLiability>>,
}

#[derive(Serialize)]
struct GetLiabilitiesRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetLiabilitiesOptions<'a>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct GetLiabilitiesOptions<'a> {
    /// A list of account_ids to retrieve for the Item
    pub account_ids: &'a [&'a str],
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetLiabilitiesResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// The accounts for which transaction history is being fetched.
    pub accounts: Vec<Account>,
    /// Metadata about the Item.
    pub item: Item,
    /// An object containing liability accounts
    pub liabilities: Liabilities,
}

impl Client {
    /// Retrieve Liabilities data.
    ///
    /// The /liabilities/get endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type credit with account subtype credit card or paypal, and account type loan with account subtype student or mortgage. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the account_filter parameter when creating a Link token.
    ///
    /// The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling /liabilities/get.
    ///
    /// * `access_token` - The access token associated with the Item data is being requested for.
    /// * `options` - An optional object to filter /liabilities/get results.
    pub async fn get_liabilities<'a>(
        &self,
        access_token: &str,
        options: Option<GetLiabilitiesOptions<'a>>,
    ) -> Result<GetLiabilitiesResponse> {
        self.send_request(
            "liabilities/get",
            &GetLiabilitiesRequest {
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
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION};

    #[tokio::test]
    async fn test_get_liabilities() {
        let client = get_test_client();
        let sandbox_resp = client
            .create_sandbox_public_token(SANDBOX_INSTITUTION, &["liabilities"])
            .await
            .unwrap();
        let token_resp = client
            .exchange_public_token(&sandbox_resp.public_token)
            .await
            .unwrap();
        let resp = client
            .get_liabilities(&token_resp.access_token, None)
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert_eq!(resp.liabilities.credit.unwrap().len(), 1);
        assert_eq!(resp.liabilities.mortgage.unwrap().len(), 1);
        assert_eq!(resp.liabilities.student.unwrap().len(), 1);

        let resp = client
            .get_liabilities(
                &token_resp.access_token,
                Some(GetLiabilitiesOptions {
                    account_ids: &[&resp.accounts[7].account_id],
                }),
            )
            .await
            .unwrap();
        assert_ne!(resp.accounts.len(), 0);
        assert!(resp.liabilities.credit.is_none());
        assert!(resp.liabilities.mortgage.is_none());
        assert_eq!(resp.liabilities.student.unwrap().len(), 1);
    }
}
