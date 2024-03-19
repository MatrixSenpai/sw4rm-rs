#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WelcomeScreenPatchRequestPartial {
    pub description: !,
    pub enabled: !,
}
