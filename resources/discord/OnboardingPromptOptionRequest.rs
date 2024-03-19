#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OnboardingPromptOptionRequest {
    pub title: !,
    pub description: !,
    pub emojiname: !,
    pub emojianimated: !,
}
