#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "candles-uploaded")]
pub struct CandlesUploadedSbModel {
    #[prost(string, tag = "1")]
    pub instrument_id: String,
}
