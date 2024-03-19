#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentDto {
    pub content: !,
    pub locale: !,
}
