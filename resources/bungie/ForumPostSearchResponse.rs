#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ForumPostSearchResponse {
    pub has_more: !,
    pub total_results: !,
    pub available_pages: !,
    pub use_total_results: !,
    pub replacement_continuation_token: !,
}
