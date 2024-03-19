#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemActionBlockDefinition {
    pub verb_name: !,
    pub overlay_icon: !,
    pub is_positive: !,
    pub verb_description: !,
    pub action_type_label: !,
    pub required_location: !,
    pub overlay_screen_name: !,
    pub required_cooldown_seconds: !,
    pub required_cooldown_hash: !,
    pub delete_on_action: !,
    pub use_on_acquire: !,
    pub consume_entire_stack: !,
}
