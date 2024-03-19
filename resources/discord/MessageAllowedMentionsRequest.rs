#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageAllowedMentionsRequest {
    pub replieduser: !,
}
