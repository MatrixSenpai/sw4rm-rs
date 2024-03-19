#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokensPartnerOfferSkuHistoryResponse {
    pub localized_description: !,
    pub localized_name: !,
    pub sku_identifier: !,
    pub all_offers_applied: !,
    pub claim_date: !,
    pub transaction_id: !,
}
