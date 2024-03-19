#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MyGuildResponse {
    pub name: !,
    pub approximatepresencecount: !,
    pub id: !,
    pub owner: !,
    pub permissions: !,
    pub icon: !,
    pub approximatemembercount: !,
}
