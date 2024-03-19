#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyQuestsDestinyQuestStatus {
    pub vendor_hash: !,
    pub step_hash: !,
    pub item_instance_id: !,
    pub started: !,
    pub redeemed: !,
    pub quest_hash: !,
    pub completed: !,
    pub tracked: !,
}
