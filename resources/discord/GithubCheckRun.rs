#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubCheckRun {
    pub conclusion: !,
    pub htmlurl: !,
    pub detailsurl: !,
    pub name: !,
}
