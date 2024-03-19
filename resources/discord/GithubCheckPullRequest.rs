#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubCheckPullRequest {
    pub number: !,
}
