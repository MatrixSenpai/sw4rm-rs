#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserModelsGetCredentialTypesForAccountResponse {
    pub is_public: !,
    pub credential_type: !,
    pub credential_as_string: !,
    pub credential_display_name: !,
}
