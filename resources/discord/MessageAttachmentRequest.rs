#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageAttachmentRequest {
    pub filename: !,
    pub isremix: !,
    pub id: !,
    pub description: !,
}
