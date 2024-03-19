#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TeamResponse {
    pub owneruserid: !,
    pub name: !,
    pub id: !,
    pub icon: !,
}
