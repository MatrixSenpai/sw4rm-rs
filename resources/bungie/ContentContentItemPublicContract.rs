#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentContentItemPublicContract {
    pub minimum_age: !,
    pub content_id: !,
    pub c_type: !,
    pub allow_comments: !,
    pub rating_image_path: !,
    pub modify_date: !,
    pub has_age_gate: !,
    pub creation_date: !,
    pub auto_english_property_fallback: !,
    pub cms_path: !,
}
