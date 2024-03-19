#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDestinyTalentNode {
    pub node_hash: !,
    pub step_index: !,
    pub node_index: !,
    pub is_activated: !,
    pub activation_grid_level: !,
    pub progress_percent: !,
    pub hidden: !,
    pub state: !,
}
