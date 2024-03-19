#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TeamMemberResponse {
    pub membershipstate: !,
    pub teamid: !,
}
