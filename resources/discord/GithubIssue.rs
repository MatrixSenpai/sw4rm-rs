#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubIssue {
    pub title: !,
    pub id: !,
    pub htmlurl: !,
    pub body: !,
    pub number: !,
}
