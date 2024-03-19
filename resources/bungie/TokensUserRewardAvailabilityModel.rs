#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokensUserRewardAvailabilityModel {
    pub is_available_for_user: !,
    pub is_unlocked_for_user: !,
}
