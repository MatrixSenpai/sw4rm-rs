#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserExactSearchRequest {
    pub display_name: !,
    pub display_name_code: !,
}
