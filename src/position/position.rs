#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide{
    Buy = 0,
    Sell = 1
}

#[derive(Clone, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderCloseReason{
    ClientCommand = 0,
    So = 1,
    Sl = 2,
    Tp = 3,
    Canceled = 4,
    AdminAction = 5
}

#[derive(Clone,PartialEq, ::prost::Message)]
pub struct OrderMetadata{
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub value: String
}

#[derive(Clone,PartialEq, ::prost::Message)]
pub struct OrderSbModel{
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(string, tag = "3")]
    pub instrument: String,
    #[prost(double, tag = "4")]
    pub invest_amount: f64,
    #[prost(double, tag = "5")]
    pub leverage: f64,
    #[prost(uint64, tag = "6")]
    pub created: u64,
    #[prost(message, tag = "7")]
    pub open_price: Option<f64>,
    #[prost(enumeration = "OrderSide", tag = "8")]
    pub side: i32,
    #[prost(message, tag = "9")]
    pub tp_percent: Option<f64>,
    #[prost(message, tag = "10")]
    pub tp_rate: Option<f64>,
    #[prost(message, tag = "11")]
    pub sl_percent: Option<f64>,
    #[prost(message, tag = "12")]
    pub sl_rate: Option<f64>,
    #[prost(uint64, tag = "13")]
    pub last_update_date: u64,
    #[prost(message, tag = "14")]
    pub create_process_id: Option<String>,
    #[prost(string, tag = "15")]
    pub last_update_process_id: String,
    #[prost(message, tag = "16")]
    pub profit: Option<f64>,
    #[prost(message, tag = "17")]
    pub close_date: Option<u64>,
    #[prost(message, tag = "18")]
    pub close_reason: Option<i32>,
    #[prost(message, tag = "19")]
    pub close_price: Option<f64>,
    #[prost(message, repeated, tag = "20")]
    pub metadata: Vec<OrderMetadata>,
    #[prost(message, tag = "21")]
    pub cancel_order_date: Option<u64>,
    #[prost(message, tag = "22")]
    pub cancel_order_process_id: Option<String>,
    #[prost(message, tag = "23")]
    pub close_process_id: Option<String>,
}

#[derive(PartialEq, ::prost::Message)]
pub struct PositionPersistenceEvent{
    #[prost(int64, tag = "1")]
    pub date_time: i64,
    #[prost(string, tag = "2")]
    pub process_id: String,
    #[prost(message, tag = "3")]
    pub update_position: Option<OrderSbModel>,
    #[prost(message, tag = "4")]
    pub close_position: Option<OrderSbModel>,
    #[prost(message, tag = "5")]
    pub create_position: Option<OrderSbModel>
}