#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyMilestonesDestinyMilestoneRewardEntry {
    pub reward_entry_hash: !,
    pub earned: !,
    pub redeemed: !,
}
