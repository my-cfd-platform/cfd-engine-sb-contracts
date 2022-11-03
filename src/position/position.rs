#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    Buy = 0,
    Sell = 1,
}

#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderCloseReason {
    ClientCommand = 0,
    So = 1,
    Sl = 2,
    Tp = 3,
    Canceled = 4,
    AdminAction = 5,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderMetadata {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderSbModel {
    #[prost(int64, tag = "1")]
    pub create_date: i64,
    #[prost(string, tag = "2")]
    pub id: String,
    #[prost(string, tag = "3")]
    pub wallet_id: String,
    #[prost(string, tag = "4")]
    pub instrument: String,
    #[prost(double, tag = "5")]
    pub invest_amount: f64,
    #[prost(int32, tag = "6")]
    pub leverage: i32,
    #[prost(uint64, tag = "7")]
    pub created: u64,
    #[prost(message, tag = "8")]
    pub open_price: Option<f64>,
    #[prost(enumeration = "OrderSide", tag = "9")]
    pub side: i32,
    #[prost(message, tag = "10")]
    pub tp_in_instrument_price: Option<f64>,
    #[prost(message, tag = "11")]
    pub tp_in_currency: Option<f64>,
    #[prost(message, tag = "12")]
    pub sl_in_instrument_price: Option<f64>,
    #[prost(message, tag = "13")]
    pub sl_in_currency: Option<f64>,
    #[prost(uint64, tag = "14")]
    pub last_update_date: u64,
    #[prost(string, tag = "15")]
    pub create_process_id: String,
    #[prost(string, tag = "16")]
    pub last_update_process_id: String,
    #[prost(message, tag = "17")]
    pub profit: Option<f64>,
    #[prost(message, tag = "18")]
    pub close_date: Option<u64>,
    #[prost(message, tag = "19")]
    pub close_reason: Option<i32>,
    #[prost(message, tag = "20")]
    pub close_price: Option<f64>,
    #[prost(message, repeated, tag = "21")]
    pub metadata: Vec<OrderMetadata>,
    #[prost(message, tag = "22")]
    pub cancel_order_date: Option<u64>,
    #[prost(message, tag = "23")]
    pub cancel_order_process_id: Option<String>,
    #[prost(message, tag = "24")]
    pub close_process_id: Option<String>,
}

#[derive(PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "engine-persistence")]
pub struct PositionPersistenceEvent {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(message, repeated, tag = "2")]
    pub update_position: Vec<OrderSbModel>,
    #[prost(message, repeated, tag = "3")]
    pub close_position: Vec<OrderSbModel>,
    #[prost(message, repeated, tag = "4")]
    pub create_position: Vec<OrderSbModel>,
}

#[derive(PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "engine-persistence-demo")]
pub struct PositionPersistenceEventDemo {
    #[prost(string, tag = "1")]
    pub process_id: String,
    #[prost(message, repeated, tag = "2")]
    pub update_position: Vec<OrderSbModel>,
    #[prost(message, repeated, tag = "3")]
    pub close_position: Vec<OrderSbModel>,
    #[prost(message, repeated, tag = "4")]
    pub create_position: Vec<OrderSbModel>,
}
