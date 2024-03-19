#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MetadataDto {
    pub matchid: !,
    pub dataversion: !,
}
