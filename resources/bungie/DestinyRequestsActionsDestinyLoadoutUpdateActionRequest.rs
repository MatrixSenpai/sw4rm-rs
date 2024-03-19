#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyLoadoutUpdateActionRequest {
    pub color_hash: !,
    pub character_id: !,
    pub membership_type: !,
    pub loadout_index: !,
    pub icon_hash: !,
    pub name_hash: !,
}
