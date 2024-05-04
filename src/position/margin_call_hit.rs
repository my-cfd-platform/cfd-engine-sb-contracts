use serde::{Deserialize, Serialize};
use service_sdk::my_service_bus;
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;
#[derive(PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "margin-call-hit")]
pub struct PositionManagerPositionMarginCallHit {
    #[prost(string, tag = "1")]
    pub position_id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub account_id: String,
    #[prost(double, tag = "4")]
    pub margin_call_percent: f64,
    #[prost(double, optional, tag = "5")]
    pub topping_up_amount: Option<f64>,
}