use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

#[derive(Serialize)]
struct GetDepositSwitchRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    deposit_switch_id: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetDepositSwitchResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///The ID of the deposit switch
    pub deposit_switch: String,
    /// The ID of the bank account the direct deposit was switched to
    pub target_account_id: Option<String>,
    /// The ID of the Item the direct deposit was switched to.
    pub target_item_id: Option<String>,
    /// The state of the deposit switch.
    /// Possible values: initialized, completed, error
    pub state: String,
    /// When true, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always null if the deposit switch has not been completed.
    pub account_has_multiple_allocations: Option<bool>,
    /// When true, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When false, user’s direct deposit is allocated as a percent or amount. Always null if the deposit switch has not been completed.
    pub is_allocated_remainder: Option<bool>,
    /// The percentage of direct deposit allocated to the target account. Always null if the target account is not allocated a percentage or if the deposit switch has not been completed or if is_allocated_remainder is true.
    pub percent_allocated: Option<i32>,
    /// The dollar amount of direct deposit allocated to the target account. Always null if the target account is not allocated an amount or if the deposit switch has not been completed.
    pub amount_allocated: Option<f64>,
    /// ISO8601 date the deposit switch was created.
    pub date_created: NaiveDate,
    /// ISO8601 date the deposit switch was completed. Always null if the deposit switch has not been completed.
    pub date_completed: Option<NaiveDate>,
}

#[derive(Serialize)]
struct CreateDepositSwitchRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    target_access_token: &'a str,
    target_account_id: &'a str,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateDepositSwitchResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
}

impl Client {
    /// Retrieve a deposit switch.
    ///
    /// This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.
    ///
    /// * `deposit_switch_id` - The ID of the deposit switch
    pub async fn get_deposit_switch<'a>(
        &self,
        deposit_switch_id: &str,
    ) -> Result<GetDepositSwitchResponse> {
        self.send_request(
            "deposit_switch/get",
            &GetDepositSwitchRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                deposit_switch_id,
            },
        )
        .await
    }

    /// Create a deposit switch.
    ///
    /// This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.
    ///
    /// * `target_account_id` - Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit.
    /// * `target_access_token` - Access token for the target Item, typically provided in the Import Item response.
    pub async fn create_deposit_switch<'a>(
        &self,
        target_account_id: &str,
        target_access_token: &str,
    ) -> Result<CreateDepositSwitchResponse> {
        self.send_request(
            "deposit_switch/create",
            &CreateDepositSwitchRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                target_account_id,
                target_access_token,
            },
        )
        .await
    }
}
