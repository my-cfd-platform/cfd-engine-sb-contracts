use serde::{Deserialize, Serialize};
use service_sdk::my_service_bus::macros::my_sb_entity_protobuf_model;
use service_sdk::my_service_bus;

#[derive(PartialEq, ::prost::Message, Serialize, Deserialize)]
#[my_sb_entity_protobuf_model(topic_id = "topping-up-event")]
pub struct PositionToppingUpEvent {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(string, tag = "2")]
    pub position_id: String,
    #[prost(string, tag = "3")]
    pub trader_id: String,
    #[prost(string, tag = "4")]
    pub account_id: String,
    #[prost(double, tag = "5")]
    pub delta: f64,
}