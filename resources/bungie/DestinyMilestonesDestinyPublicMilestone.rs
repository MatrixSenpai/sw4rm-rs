#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyMilestonesDestinyPublicMilestone {
    pub start_date: !,
    pub milestone_hash: !,
    pub end_date: !,
    pub order: !,
}
