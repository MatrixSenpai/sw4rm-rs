#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VoiceRegionResponse {
    pub deprecated: !,
    pub name: !,
    pub id: !,
    pub optimal: !,
    pub custom: !,
}
