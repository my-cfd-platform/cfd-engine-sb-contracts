#[derive(PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "bidask")]
pub struct BidAsk {
    #[prost(string, tag = "1")]
    pub instrument_id: String,
    #[prost(double, tag = "2")]
    pub bid: f64,
    #[prost(double, tag = "3")]
    pub ask: f64,
    #[prost(uint64, tag = "4")]
    pub date_time: u64,
}
