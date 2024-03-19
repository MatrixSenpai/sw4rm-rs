#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub heading: !,
    pub createdat: !,
    pub updatedat: !,
    pub author: !,
    pub content: !,
    pub id: !,
    pub severity: !,
}
