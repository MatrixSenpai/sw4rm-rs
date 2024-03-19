#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SelectOptionResponse {
    pub label: !,
    pub description: !,
    pub default: !,
    pub value: !,
}
