#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentNewsArticleRssResponse {
    pub result_count_this_page: !,
    pub next_pagination_token: !,
    pub category_filter: !,
    pub current_pagination_token: !,
}
