#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGuildInviteRequest {
    pub maxage: !,
    pub temporary: !,
    pub maxuses: !,
    pub unique: !,
}
