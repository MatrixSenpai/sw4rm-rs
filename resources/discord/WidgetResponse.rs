#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WidgetResponse {
    pub id: !,
    pub instantinvite: !,
    pub presencecount: !,
    pub name: !,
}
