#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfFireteamResponse {
    pub replacement_continuation_token: !,
    pub use_total_results: !,
    pub total_results: !,
    pub has_more: !,
}
