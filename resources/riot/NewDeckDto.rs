#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NewDeckDto {
    pub name: !,
    pub code: !,
}
