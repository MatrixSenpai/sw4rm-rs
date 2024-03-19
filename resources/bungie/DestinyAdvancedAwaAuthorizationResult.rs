#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DestinyAdvancedAwaAuthorizationResult {
    pub action_token: !,
    pub membership_type: !,
    pub user_selection: !,
    pub developer_note: !,
    pub maximum_number_of_uses: !,
    pub valid_until: !,
    pub type: !,
    pub response_reason: !,
}
