#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MiniSeriesDto {
    pub losses: !,
    pub progress: !,
    pub target: !,
    pub wins: !,
}
