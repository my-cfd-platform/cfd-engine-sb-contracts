use crate::{CrossMarginOrderMetadata, CrossMarginOrderSide};
use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();

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
    #[prost(double, tag = "7")]
    pub leverage: f64,
    #[prost(double, tag = "8")]
    pub stop_out_percent: f64,
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
}

#[derive(PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "cross-margin-pending-order-persistence")]
pub struct CrossMarginPositionPersistenceEvent {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(message, tag = "2")]
    pub update_position: Option<CrossMarginPendingOrderSbModel>,
    #[prost(message, tag = "3")]
    pub close_position: Option<CrossMarginPendingOrderSbModel>,
    #[prost(message, tag = "4")]
    pub create_position: Option<CrossMarginPendingOrderSbModel>,
}
