#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentItemDto {
    pub name: !,
    pub id: !,
    pub asset_path: !,
    pub asset_name: !,
}
