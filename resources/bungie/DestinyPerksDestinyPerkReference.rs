#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyPerksDestinyPerkReference {
    pub is_active: !,
    pub perk_hash: !,
    pub icon_path: !,
    pub visible: !,
}
