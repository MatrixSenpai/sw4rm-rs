#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Service {
    pub slug: !,
    pub status: !,
    pub name: !,
}
