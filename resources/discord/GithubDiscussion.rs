#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubDiscussion {
    pub title: !,
    pub number: !,
    pub answerhtmlurl: !,
    pub htmlurl: !,
    pub body: !,
}
