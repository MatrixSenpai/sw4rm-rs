#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyAdvancedAwaInitializeResponse {
    pub correlation_id: !,
    pub sent_to_self: !,
}
