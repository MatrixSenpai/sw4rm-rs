#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RichEmbedProvider {
    pub name: !,
    pub url: !,
}
