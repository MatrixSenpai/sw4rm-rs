#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageInteractionResponse {
    pub namelocalized: !,
    pub id: !,
    pub type: !,
    pub name: !,
}
