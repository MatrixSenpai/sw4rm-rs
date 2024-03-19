#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubReview {
    pub htmlurl: !,
    pub state: !,
    pub body: !,
}
