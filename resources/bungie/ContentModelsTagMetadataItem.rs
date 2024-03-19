#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentModelsTagMetadataItem {
    pub is_default: !,
    pub description: !,
    pub tag_text: !,
    pub name: !,
}
