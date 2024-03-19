#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyComponentsVendorsDestinyVendorSaleItemBaseComponent {
    pub vendor_item_index: !,
    pub api_purchasable: !,
    pub override_next_refresh_date: !,
    pub item_hash: !,
    pub override_style_item_hash: !,
    pub quantity: !,
}
