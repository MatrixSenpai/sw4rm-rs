#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemStatBlockDefinition {
    pub disable_primary_stat_display: !,
    pub has_displayable_stats: !,
    pub primary_base_stat_hash: !,
    pub stat_group_hash: !,
}
