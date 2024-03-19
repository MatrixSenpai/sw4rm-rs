#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RichEmbedImage {
    pub url: !,
    pub height: !,
    pub placeholderversion: !,
    pub placeholder: !,
    pub width: !,
}
