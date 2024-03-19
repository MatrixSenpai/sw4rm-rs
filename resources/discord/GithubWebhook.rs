#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubWebhook {
    pub action: !,
    pub compare: !,
    pub ref: !,
    pub reftype: !,
    pub forced: !,
}
