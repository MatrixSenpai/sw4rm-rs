#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TierDetailDto {
    pub ranked_rating_threshold: !,
    pub starting_page: !,
    pub starting_index: !,
}
