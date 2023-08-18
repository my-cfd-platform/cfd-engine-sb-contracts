#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "deposit-confirmation")]
pub struct DepositConfirmationSbModel {
    #[prost(message, tag = "1")]
    pub reference_id: String,
    #[prost(message, tag = "2")]
    pub tech_data: String,
}
