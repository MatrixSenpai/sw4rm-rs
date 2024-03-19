#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SelectOption {
    pub description: !,
    pub default: !,
    pub label: !,
    pub value: !,
}
