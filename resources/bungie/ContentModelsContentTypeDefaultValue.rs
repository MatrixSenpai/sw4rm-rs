#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentModelsContentTypeDefaultValue {
    pub when_clause: !,
    pub when_value: !,
    pub default_value: !,
}
