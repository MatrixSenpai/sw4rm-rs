#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyLoadoutActionRequest {
    pub character_id: !,
    pub loadout_index: !,
    pub membership_type: !,
}
