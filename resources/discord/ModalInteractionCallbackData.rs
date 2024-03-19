#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModalInteractionCallbackData {
    pub customid: !,
    pub title: !,
}
