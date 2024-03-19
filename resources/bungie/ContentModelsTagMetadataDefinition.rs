#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentModelsTagMetadataDefinition {
    pub is_required: !,
    pub datatype: !,
    pub name: !,
    pub description: !,
    pub order: !,
}
