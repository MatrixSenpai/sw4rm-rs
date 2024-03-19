#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokensPartnerOfferClaimRequest {
    pub partner_offer_id: !,
    pub bungie_net_membership_id: !,
    pub transaction_id: !,
}
