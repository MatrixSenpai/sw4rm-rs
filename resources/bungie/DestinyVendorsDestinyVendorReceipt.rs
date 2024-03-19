#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyVendorsDestinyVendorReceipt {
    pub refund_policy: !,
    pub purchased_by_character_id: !,
    pub license_unlock_hash: !,
    pub sequence_number: !,
    pub expires_on: !,
    pub time_to_expiration: !,
}
