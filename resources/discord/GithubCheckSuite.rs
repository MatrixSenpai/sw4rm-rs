#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubCheckSuite {
    pub conclusion: !,
    pub headsha: !,
    pub headbranch: !,
}
