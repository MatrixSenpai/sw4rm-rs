#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResultOfFireteamSummary {
    pub total_results: !,
    pub has_more: !,
    pub replacement_continuation_token: !,
    pub use_total_results: !,
}
