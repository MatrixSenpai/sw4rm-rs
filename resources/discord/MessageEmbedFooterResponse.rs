#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageEmbedFooterResponse {
    pub iconurl: !,
    pub proxyiconurl: !,
    pub text: !,
}
