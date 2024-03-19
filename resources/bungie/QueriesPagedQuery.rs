#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QueriesPagedQuery {
    pub current_page: !,
    pub items_per_page: !,
    pub request_continuation_token: !,
}
