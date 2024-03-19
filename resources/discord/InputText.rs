#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InputText {
    pub maxlength: !,
    pub type: !,
    pub minlength: !,
    pub label: !,
    pub customid: !,
    pub style: !,
    pub value: !,
    pub placeholder: !,
    pub required: !,
}
