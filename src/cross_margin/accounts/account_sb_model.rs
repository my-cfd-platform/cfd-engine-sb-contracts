use serde::{Deserialize, Serialize};

use crate::CrossMarginAccountBalanceUpdateSbModel;

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "cross-margin-account-persist-events")]
#[derive(Serialize, Deserialize)]
pub struct CrossMarginAccountPersistEvent {
    #[prost(message, tag = "1")]
    pub add_account_event: Option<CrossMarginAccountSbModel>,
    #[prost(message, tag = "2")]
    pub update_account_event: Option<CrossMarginAccountBalanceUpdateSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginAccountSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub currency: String,
    #[prost(double, tag = "4")]
    pub balance: f64,
    #[prost(bool, tag = "5")]
    pub trading_disabled: bool,
    #[prost(double, tag = "6")]
    pub leverage: f64,
    #[prost(string, tag = "7")]
    pub trading_group: String,
    #[prost(double, tag = "8")]
    pub stop_out: f64,
    #[prost(uint64, tag = "9")]
    pub create_date: u64,
    #[prost(uint64, tag = "10")]
    pub last_update_date: u64,
    #[prost(string, tag = "11")]
    pub last_update_process_id: String,
    #[prost(string, tag = "12")]
    pub create_process_id: String,
}
