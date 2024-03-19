#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationsDatapoint {
    pub time: !,
    pub count: !,
}
