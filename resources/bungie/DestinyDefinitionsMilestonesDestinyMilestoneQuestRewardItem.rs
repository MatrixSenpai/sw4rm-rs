#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsMilestonesDestinyMilestoneQuestRewardItem {
    pub item_hash: !,
    pub vendor_item_index: !,
    pub item_instance_id: !,
    pub has_conditional_visibility: !,
    pub vendor_hash: !,
    pub quantity: !,
}
