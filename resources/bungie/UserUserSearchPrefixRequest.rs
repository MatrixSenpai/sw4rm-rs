#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserUserSearchPrefixRequest {
    pub display_name_prefix: !,
}
