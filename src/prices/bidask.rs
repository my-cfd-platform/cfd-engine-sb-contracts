#[derive(PartialEq, ::prost::Message)]
pub struct BidAsk{
    #[prost(string, tag = "1")]
    pub instrument_id: String,
    #[prost(double, tag = "2")]
    pub bid: f64,
    #[prost(double, tag = "3")]
    pub ask: f64,
    #[prost(uint64, tag = "4")]
    pub date_time: u64
}