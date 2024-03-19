#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDyeReference {
    pub dye_hash: !,
    pub channel_hash: !,
}
