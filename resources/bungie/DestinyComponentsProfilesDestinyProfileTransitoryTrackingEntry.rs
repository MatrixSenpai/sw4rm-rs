#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyComponentsProfilesDestinyProfileTransitoryTrackingEntry {
    pub questline_item_hash: !,
    pub activity_hash: !,
    pub location_hash: !,
    pub item_hash: !,
    pub objective_hash: !,
    pub tracked_date: !,
}
