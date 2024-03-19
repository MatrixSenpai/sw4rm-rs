#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WidgetChannel {
    pub id: !,
    pub name: !,
    pub position: !,
}
