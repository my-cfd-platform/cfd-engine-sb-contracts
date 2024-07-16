use crate::{CrossMarginOrderMetadata, CrossMarginOrderSide, CrossMarginPendingOrderType};
use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, Debug, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum CrossMarginPendingOrderType {
    Stop = 0,
    Limit = 1,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginPendingOrderSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub account_id: String,
    #[prost(string, tag = "4")]
    pub asset_pair: String,
    #[prost(double, tag = "5")]
    pub lots_amount: f64,
    #[prost(enumeration = "CrossMarginOrderSide", tag = "6")]
    pub side: i32,
    #[prost(uint64, tag = "9")]
    pub create_date: u64,
    #[prost(message, tag = "10")]
    pub tp_in_instrument_price: Option<f64>,
    #[prost(message, tag = "11")]
    pub tp_in_currency: Option<f64>,
    #[prost(message, tag = "12")]
    pub sl_in_instrument_price: Option<f64>,
    #[prost(message, tag = "13")]
    pub sl_in_currency: Option<f64>,
    #[prost(string, tag = "14")]
    pub create_process_id: String,
    #[prost(message, repeated, tag = "15")]
    pub metadata: Vec<CrossMarginOrderMetadata>,
    #[prost(uint64, tag = "16")]
    pub last_update_date: u64,
    #[prost(string, tag = "17")]
    pub last_update_process_id: String,
    #[prost(string, tag = "18")]
    pub base: String,
    #[prost(string, tag = "19")]
    pub quote: String,
    #[prost(string, tag = "20")]
    pub collateral_currency: String,
    #[prost(double, tag = "21")]
    pub desire_price: f64,
    #[prost(double, tag = "22")]
    pub lots_size: f64,
    #[prost(message, tag = "23")]
    pub execute_price: Option<f64>,
    #[prost(enumeration = "CrossMarginPendingOrderType", tag = "24")]
    pub order_type: i32,
    #[prost(double, tag = "25")]
    pub init_price: f64,
}

#[derive(PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "cross-margin-pending-order-persistence")]
pub struct CrossMarginPendingOrderPersistenceEvent {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(message, tag = "2")]
    pub created: Option<CrossMarginPendingOrderSbModel>,
    #[prost(message, tag = "3")]
    pub canceled: Option<CrossMarginPendingOrderSbModel>,
    #[prost(message, tag = "4")]
    pub failed: Option<CrossMarginPendingOrderSbModel>,
    #[prost(message, tag = "5")]
    pub executed: Option<CrossMarginPendingOrderSbModel>,
}
