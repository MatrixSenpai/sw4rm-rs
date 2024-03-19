#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommonModelsCoreSetting {
    pub is_default: !,
    pub display_name: !,
    pub summary: !,
    pub image_path: !,
    pub identifier: !,
}
