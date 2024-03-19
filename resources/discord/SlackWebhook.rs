#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SlackWebhook {
    pub text: !,
    pub iconurl: !,
    pub username: !,
}
