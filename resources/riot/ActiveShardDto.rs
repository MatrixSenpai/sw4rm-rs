#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveShardDto {
    pub active_shard: !,
    pub game: !,
    pub puuid: !,
}
