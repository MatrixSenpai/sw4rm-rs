#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pet {
    pub id: !,
    pub name: !,
    pub tag: !,
}
