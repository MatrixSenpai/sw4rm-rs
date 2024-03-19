#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserEMailSettingSubscriptionLocalization {
    pub known_user_action_text: !,
    pub registered_user_description: !,
    pub unregistered_user_description: !,
    pub unknown_user_action_text: !,
    pub title: !,
    pub unknown_user_description: !,
    pub description: !,
}
