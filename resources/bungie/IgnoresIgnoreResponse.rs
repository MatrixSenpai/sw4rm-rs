#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IgnoresIgnoreResponse {
    pub ignore_flags: !,
    pub is_ignored: !,
}
