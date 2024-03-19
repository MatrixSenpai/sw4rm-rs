#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageComponentInputTextResponse {
    pub customid: !,
    pub style: !,
    pub required: !,
    pub minlength: !,
    pub maxlength: !,
    pub placeholder: !,
    pub label: !,
    pub type: !,
    pub value: !,
}
