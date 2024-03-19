#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ReplyMessageReferenceRequest {
    pub failifnotexists: !,
    pub messageid: !,
}
