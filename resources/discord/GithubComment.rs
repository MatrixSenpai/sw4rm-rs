#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubComment {
    pub id: !,
    pub commitid: !,
    pub htmlurl: !,
    pub body: !,
}
