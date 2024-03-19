#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubUser {
    pub htmlurl: !,
    pub avatarurl: !,
    pub login: !,
    pub id: !,
}
