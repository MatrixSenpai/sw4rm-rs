#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RichEmbedField {
    pub inline: !,
    pub value: !,
    pub name: !,
}
