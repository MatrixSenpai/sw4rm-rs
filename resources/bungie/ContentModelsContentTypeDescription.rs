#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentModelsContentTypeDescription {
    pub type_of: !,
    pub auto_english_property_fallback: !,
    pub suppress_cms_path: !,
    pub bulk_uploadable: !,
    pub reminder: !,
    pub content_description: !,
    pub preview_image: !,
    pub force_identifier_binding: !,
    pub bind_identifier_to_property: !,
    pub c_type: !,
    pub bound_regex: !,
    pub allow_comments: !,
    pub name: !,
    pub priority: !,
    pub show_in_content_editor: !,
}
