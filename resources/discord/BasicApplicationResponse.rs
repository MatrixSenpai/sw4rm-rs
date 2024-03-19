#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BasicApplicationResponse {
    pub name: !,
    pub description: !,
    pub id: !,
    pub coverimage: !,
    pub icon: !,
}
