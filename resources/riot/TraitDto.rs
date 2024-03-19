#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TraitDto {
    pub name: !,
    pub style: !,
    pub tiercurrent: !,
    pub numunits: !,
    pub tiertotal: !,
}
