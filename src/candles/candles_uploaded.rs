use serde::{Deserialize, Serialize};

service_sdk::macros::use_my_sb_entity_protobuf_model!();
#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "candles-uploaded")]
#[derive(Serialize, Deserialize)]
pub struct CandlesUploadedSbModel {
    #[prost(string, tag = "1")]
    pub instrument_id: String,
}
