#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyVendorActionDefinition {
    pub verb: !,
    pub is_positive: !,
    pub action_id: !,
    pub description: !,
    pub execute_seconds: !,
    pub action_hash: !,
    pub name: !,
    pub icon: !,
    pub auto_perform_action: !,
}
