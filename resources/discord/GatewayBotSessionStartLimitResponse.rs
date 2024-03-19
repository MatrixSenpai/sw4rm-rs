#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatewayBotSessionStartLimitResponse {
    pub maxconcurrency: !,
    pub total: !,
    pub resetafter: !,
    pub remaining: !,
}
