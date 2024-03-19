#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyAdvancedAwaPermissionRequested {
    pub type: !,
    pub character_id: !,
    pub membership_type: !,
    pub affected_item_id: !,
}
