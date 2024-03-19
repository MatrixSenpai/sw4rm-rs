#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyDefinitionsDestinyVendorCategoryEntryDefinition {
    pub hide_from_regular_purchase: !,
    pub is_display_only: !,
    pub quantity_available: !,
    pub disabled_description: !,
    pub reset_offset_minutes_override: !,
    pub show_unavailable_items: !,
    pub category_hash: !,
    pub hide_if_no_currency: !,
    pub category_index: !,
    pub sort_value: !,
    pub display_title: !,
    pub reset_interval_minutes_override: !,
    pub buy_string_override: !,
    pub is_preview: !,
}
