#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupEditAction {
    pub allow_chat: !,
    pub about: !,
    pub homepage: !,
    pub membership_option: !,
    pub avatar_image_index: !,
    pub theme: !,
    pub callsign: !,
    pub name: !,
    pub is_public_topic_admin_only: !,
    pub chat_security: !,
    pub tags: !,
    pub is_public: !,
    pub motto: !,
    pub enable_invitation_messaging_for_admins: !,
    pub default_publicity: !,
    pub locale: !,
}
