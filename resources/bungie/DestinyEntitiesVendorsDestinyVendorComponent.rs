#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyEntitiesVendorsDestinyVendorComponent {
    pub can_purchase: !,
    pub next_refresh_date: !,
    pub seasonal_rank: !,
    pub enabled: !,
    pub vendor_location_index: !,
    pub vendor_hash: !,
}
