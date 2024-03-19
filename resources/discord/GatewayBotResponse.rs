#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatewayBotResponse {
    pub shards: !,
    pub url: !,
}
