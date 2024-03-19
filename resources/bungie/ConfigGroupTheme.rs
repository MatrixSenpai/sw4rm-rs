#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigGroupTheme {
    pub folder: !,
    pub name: !,
    pub description: !,
}
