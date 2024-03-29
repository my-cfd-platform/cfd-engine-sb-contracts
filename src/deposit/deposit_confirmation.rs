use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "deposit-confirmation")]
#[derive(Serialize, Deserialize)]
pub struct DepositConfirmationSbModel {
    #[prost(string, tag = "1")]
    pub payment_system_id: String,
    #[prost(string, tag = "2")]
    pub reference_id: String,
    #[prost(string, tag = "3")]
    pub tech_data: String,
}
