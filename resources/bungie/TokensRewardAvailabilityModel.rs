#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokensRewardAvailabilityModel {
    pub shopify_end_date: !,
    pub redemption_end_date: !,
    pub game_earn_by_date: !,
    pub decrypted_token: !,
    pub has_offer: !,
    pub offer_applied: !,
    pub is_loyalty_reward: !,
    pub is_offer: !,
    pub has_existing_code: !,
}
