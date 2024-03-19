#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompanionDto {
    pub item_id: !,
    pub skin_id: !,
    pub species: !,
    pub content_id: !,
}
