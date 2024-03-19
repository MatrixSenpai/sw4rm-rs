#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemObjectiveBlockDefinition {
    pub narrative: !,
    pub objective_verb_name: !,
    pub quest_type_identifier: !,
    pub quest_type_hash: !,
    pub require_full_objective_completion: !,
    pub questline_item_hash: !,
    pub display_as_stat_tracker: !,
}
