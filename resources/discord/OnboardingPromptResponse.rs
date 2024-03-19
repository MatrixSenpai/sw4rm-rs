#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OnboardingPromptResponse {
    pub id: !,
    pub required: !,
    pub inonboarding: !,
    pub type: !,
    pub title: !,
    pub singleselect: !,
}
