use serde::{Deserialize, Serialize};
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;
use service_sdk::my_service_bus;
#[derive(PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "margin-call-hit")]
pub struct PositionManagerPositionMarginCallHit{
    #[prost(string, tag = "1")]
    pub position_id: String,
    #[prost(string, tag = "2")]
    pub trader_id: String,
    #[prost(string, tag = "3")]
    pub account_id: String,
    #[prost(double, optional, tag = "4")]
    pub topping_up_amount: Option<f64>,
    #[prost(double, tag = "5")]
    pub margin_call_percent: f64,
}