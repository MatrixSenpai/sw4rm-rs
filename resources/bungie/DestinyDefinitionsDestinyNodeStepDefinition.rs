#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyNodeStepDefinition {
    pub damage_type: !,
    pub step_index: !,
    pub damage_type_hash: !,
    pub interaction_description: !,
    pub can_activate_next_step: !,
    pub start_progression_bar_at_progress: !,
    pub affects_quality: !,
    pub affects_level: !,
    pub node_step_hash: !,
    pub next_step_index: !,
    pub is_next_step_random: !,
}
