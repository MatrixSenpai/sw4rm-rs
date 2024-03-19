#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageRoleSubscriptionDataResponse {
    pub isrenewal: !,
    pub rolesubscriptionlistingid: !,
    pub tiername: !,
    pub totalmonthssubscribed: !,
}
