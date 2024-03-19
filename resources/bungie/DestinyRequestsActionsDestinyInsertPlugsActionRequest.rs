#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyInsertPlugsActionRequest {
    pub item_instance_id: !,
    pub action_token: !,
    pub character_id: !,
    pub membership_type: !,
}
