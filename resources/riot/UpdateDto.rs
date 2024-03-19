#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateDto {
    pub createdat: !,
    pub updatedat: !,
    pub id: !,
    pub author: !,
    pub publish: !,
}
