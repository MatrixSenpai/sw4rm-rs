#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubRepository {
    pub id: !,
    pub fullname: !,
    pub htmlurl: !,
    pub name: !,
}
