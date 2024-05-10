use serde::{Serialize, Deserialize};

use crate::AccountSbModel;

service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "cross-margin-account-balance-updated-event")]
#[derive(Serialize, Deserialize)]
pub struct CrossMarginAccountBalanceUpdateSbModel {
    #[prost(message, tag = "1")]
    pub account_after_update: Option<AccountSbModel>,
    #[prost(message, tag = "2")]
    pub operation: Option<CrossMarginAccountBalanceUpdateOperationSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct CrossMarginAccountBalanceUpdateOperationSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub account_id: String,
    #[prost(enumeration = "CrossMarginAccountBalanceUpdateOperationType", tag = "4")]
    pub operation_type: i32,
    #[prost(message, tag = "5")]
    pub process_id: Option<String>,
    #[prost(double, tag = "6")]
    pub delta: f64,
    #[prost(uint64, tag = "7")]
    pub date_time_unix_ms: u64,
    #[prost(message, tag = "8")]
    pub comment: Option<String>,
    #[prost(message, tag = "9")]
    pub reference_operation_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
pub enum CrossMarginAccountBalanceUpdateOperationType {
    Trading = 0,
    BalanceCorrection = 1,
    Withdrawal = 2,
    Deposit = 3,
    WithdrawalCanceled = 4,
    ToppingUp = 5,
    Dividends = 6,
    Bonus = 7,
    Credit = 8,
    Voucher = 9,
}
