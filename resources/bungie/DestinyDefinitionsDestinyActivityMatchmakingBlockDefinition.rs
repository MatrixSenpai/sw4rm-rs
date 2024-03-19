#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyActivityMatchmakingBlockDefinition {
    pub max_players: !,
    pub is_matchmade: !,
    pub max_party: !,
    pub min_party: !,
    pub requires_guardian_oath: !,
}
