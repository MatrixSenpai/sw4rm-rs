#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubCommit {
    pub id: !,
    pub message: !,
    pub url: !,
}
