#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyTalentNodeDefinition {
    pub column: !,
    pub node_index: !,
    pub node_hash: !,
    pub binary_pair_node_index: !,
    pub auto_unlocks: !,
    pub is_random: !,
    pub is_random_repurchasable: !,
    pub layout_identifier: !,
    pub group_hash: !,
    pub node_style_identifier: !,
    pub last_step_repeats: !,
    pub row: !,
    pub random_start_progression_bar_at_progression: !,
    pub lore_hash: !,
    pub ignore_for_completion: !,
}
