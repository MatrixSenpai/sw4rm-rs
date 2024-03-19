#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubRelease {
    pub id: !,
    pub htmlurl: !,
    pub tagname: !,
}
