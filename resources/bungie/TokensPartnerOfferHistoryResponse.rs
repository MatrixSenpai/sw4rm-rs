#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokensPartnerOfferHistoryResponse {
    pub localized_description: !,
    pub apply_date: !,
    pub membership_type: !,
    pub localized_name: !,
    pub partner_offer_key: !,
    pub membership_id: !,
    pub is_consumable: !,
    pub quantity_applied: !,
}
