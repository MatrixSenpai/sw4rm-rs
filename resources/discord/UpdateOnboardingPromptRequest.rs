#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateOnboardingPromptRequest {
    pub required: !,
    pub singleselect: !,
    pub inonboarding: !,
    pub id: !,
    pub title: !,
}
