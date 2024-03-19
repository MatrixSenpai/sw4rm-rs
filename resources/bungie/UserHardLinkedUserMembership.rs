#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserHardLinkedUserMembership {
    pub cross_save_overridden_type: !,
    pub cross_save_overridden_membership_id: !,
    pub membership_type: !,
    pub membership_id: !,
}
