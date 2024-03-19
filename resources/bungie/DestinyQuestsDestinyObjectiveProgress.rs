#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyQuestsDestinyObjectiveProgress {
    pub progress: !,
    pub completion_value: !,
    pub destination_hash: !,
    pub activity_hash: !,
    pub complete: !,
    pub visible: !,
    pub objective_hash: !,
}
