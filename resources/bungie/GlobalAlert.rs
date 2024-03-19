#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GlobalAlert {
    pub alert_key: !,
    pub alert_type: !,
    pub alert_html: !,
    pub alert_timestamp: !,
    pub alert_link: !,
    pub alert_level: !,
}
