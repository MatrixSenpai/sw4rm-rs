#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Incident {
    pub createdat: !,
    pub id: !,
    pub active: !,
}
