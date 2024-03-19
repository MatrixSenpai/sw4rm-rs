#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserUserToUserContext {
    pub global_ignore_end_date: !,
    pub is_following: !,
}
