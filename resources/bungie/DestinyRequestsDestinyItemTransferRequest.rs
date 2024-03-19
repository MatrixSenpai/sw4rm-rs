#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyRequestsDestinyItemTransferRequest {
    pub character_id: !,
    pub transfer_to_vault: !,
    pub item_reference_hash: !,
    pub stack_size: !,
    pub item_id: !,
    pub membership_type: !,
}
