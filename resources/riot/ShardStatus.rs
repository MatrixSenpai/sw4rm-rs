#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ShardStatus {
    pub name: !,
    pub hostname: !,
    pub regiontag: !,
    pub slug: !,
}
