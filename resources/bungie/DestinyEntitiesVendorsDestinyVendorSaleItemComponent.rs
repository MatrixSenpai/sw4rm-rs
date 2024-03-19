#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyEntitiesVendorsDestinyVendorSaleItemComponent {
    pub item_hash: !,
    pub augments: !,
    pub sale_status: !,
    pub vendor_item_index: !,
    pub quantity: !,
    pub api_purchasable: !,
    pub override_style_item_hash: !,
    pub override_next_refresh_date: !,
}
