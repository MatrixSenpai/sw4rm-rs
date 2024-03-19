#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsActionsDestinyPostmasterTransferRequest {
    pub membership_type: !,
    pub item_reference_hash: !,
    pub item_id: !,
    pub stack_size: !,
    pub character_id: !,
}
