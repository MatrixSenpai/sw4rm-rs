#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyItemQualityBlockDefinition {
    pub progression_level_requirement_hash: !,
    pub infusion_category_name: !,
    pub infusion_category_hash: !,
    pub quality_level: !,
    pub current_version: !,
}
