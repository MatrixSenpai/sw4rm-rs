#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ScheduledEventUserResponse {
    pub guildscheduledeventid: !,
    pub userid: !,
}
