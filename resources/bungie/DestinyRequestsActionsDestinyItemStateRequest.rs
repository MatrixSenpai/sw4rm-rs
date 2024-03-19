#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyItemStateRequest {
    pub state: !,
    pub item_id: !,
    pub character_id: !,
    pub membership_type: !,
}
