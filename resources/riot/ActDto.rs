#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActDto {
    pub is_active: !,
    pub parent_id: !,
    pub id: !,
    pub type: !,
    pub name: !,
}
