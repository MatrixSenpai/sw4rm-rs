#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MentionSpamTriggerMetadata {
    pub mentionraidprotectionenabled: !,
    pub mentiontotallimit: !,
}
