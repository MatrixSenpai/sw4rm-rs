#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageEmbedFieldResponse {
    pub name: !,
    pub inline: !,
    pub value: !,
}
