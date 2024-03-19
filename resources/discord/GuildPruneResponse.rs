#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildPruneResponse {
    pub pruned: !,
}
