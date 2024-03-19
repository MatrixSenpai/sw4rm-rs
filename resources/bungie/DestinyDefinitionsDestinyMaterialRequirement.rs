#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyMaterialRequirement {
    pub delete_on_action: !,
    pub omit_from_requirements: !,
    pub item_hash: !,
    pub count_is_constant: !,
    pub count: !,
}
