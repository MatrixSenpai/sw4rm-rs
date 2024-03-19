#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationCommandAttachmentOption {
    pub name: !,
    pub required: !,
    pub type: !,
    pub description: !,
}
