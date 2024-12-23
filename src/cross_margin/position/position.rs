use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, Debug, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum CrossMarginOrderSide {
    Buy = 0,
    Sell = 1,
}

#[derive(Clone, Debug, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum CrossMarginOrderCloseReasonSbModel {
    ClientCommand = 0,
    StopOut = 1,
    TakeProfit = 2,
    StopLoss = 3,
    ForceClose = 4,
}

#[derive(Clone, Debug, ::prost::Enumeration, Serialize, Deserialize)]
#[repr(i32)]
pub enum CrossMarginOrderCloseReason {
    ClientCommand = 0,
    So = 1,
    Sl = 2,
    Tp = 3,
    Canceled = 4,
    AdminAction = 5,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginOrderMetadata {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginOrderSwap {
    #[prost(double, tag = "1")]
    pub amount: f64,
    #[prost(uint64, tag = "2")]
    pub date: u64,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginOrderBidAskSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(double, tag = "2")]
    pub bid: f64,
    #[prost(double, tag = "3")]
    pub ask: f64,
    #[prost(uint64, tag = "4")]
    pub date: u64,
    #[prost(string, tag = "5")]
    pub base: String,
    #[prost(string, tag = "6")]
    pub quote: String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct CrossMarginOrderSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub account_id: String,
    #[prost(string, tag = "4")]
    pub asset_pair: String,
    #[prost(double, tag = "5")]
    pub lost_amount: f64,
    #[prost(enumeration = "CrossMarginOrderSide", tag = "6")]
    pub side: i32,
    #[prost(double, tag = "7")]
    pub leverage: f64,
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
    #[prost(message, tag = "15")]
    pub profit: Option<f64>,
    #[prost(message, repeated, tag = "16")]
    pub metadata: Vec<CrossMarginOrderMetadata>,
    #[prost(uint64, tag = "17")]
    pub last_update_date: u64,
    #[prost(string, tag = "18")]
    pub last_update_process_id: String,
    #[prost(double, tag = "19")]
    pub asset_open_price: f64,
    #[prost(message, tag = "22")]
    pub asset_open_bid_ask: Option<CrossMarginOrderBidAskSbModel>,
    #[prost(uint64, tag = "20")]
    pub open_date: u64,
    #[prost(string, tag = "21")]
    pub open_process_id: String,
    #[prost(message, tag = "23")]
    pub close_date: Option<u64>,
    #[prost(
        optional,
        enumeration = "CrossMarginOrderCloseReasonSbModel",
        tag = "24"
    )]
    pub close_reason: Option<i32>,
    #[prost(message, tag = "25")]
    pub asset_close_price: Option<f64>,
    #[prost(message, tag = "26")]
    pub asset_close_bid_ask: Option<CrossMarginOrderBidAskSbModel>,
    #[prost(message, tag = "27")]
    pub close_process_id: Option<String>,
    #[prost(string, tag = "28")]
    pub base: String,
    #[prost(string, tag = "29")]
    pub quote: String,
    #[prost(string, tag = "30")]
    pub collateral_currency: String,
    #[prost(double, tag = "31")]
    pub margin_price: f64,
    #[prost(message, tag = "32")]
    pub margin_bid_ask: Option<CrossMarginOrderBidAskSbModel>,
    #[prost(optional, double, tag = "33")]
    pub profit_price: Option<f64>,
    #[prost(message, tag = "34")]
    pub profit_bid_ask: Option<CrossMarginOrderBidAskSbModel>,
    #[prost(message, repeated, tag = "35")]
    pub swaps: Vec<CrossMarginOrderSwap>,
    #[prost(double, tag = "36")]
    pub lots_amount: f64,
    #[prost(double, tag = "37")]
    pub lots_size: f64,
    #[prost(message, tag = "38")]
    pub commission: Option<f64>,
    #[prost(optional, message, tag = "39")]
    pub desire_price: Option<f64>,
    #[prost(optional, message, tag = "40")]
    pub init_price: Option<f64>,

}

#[derive(PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "cross-margin-position-persistence")]
pub struct CrossMarginPositionPersistenceEvent {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(message, tag = "2")]
    pub created: Option<CrossMarginOrderSbModel>,
    #[prost(message, tag = "3")]
    pub updated: Option<CrossMarginOrderSbModel>,
    #[prost(message, tag = "4")]
    pub closed: Option<CrossMarginOrderSbModel>,
}
