#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "account-persist-events")]
pub struct AccountPersistEvent{
    #[prost(message, tag = "1")]
    pub add_account_event: Option<AccountSbModel>,
    #[prost(message, tag = "2")]
    pub update_account_event: Option<AccountSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSbModel{
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub currency: String,
    #[prost(string, tag = "3")]
    pub trader_id: String,
    #[prost(uint64, tag = "4")]
    pub create_date: u64,
    #[prost(uint64, tag = "5")]
    pub last_update_date: u64,
    #[prost(string, tag = "6")]
    pub last_update_process_id: String,
    #[prost(double, tag = "7")]
    pub balance: f64,
    #[prost(bool, tag = "8")]
    pub trading_disabled: bool,
    #[prost(string, tag = "9")]
    pub create_process_id: String,
    #[prost(string, tag = "10")]
    pub trading_group: String,
}