#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WebhookSlackEmbed {
    pub ts: !,
    pub imageurl: !,
    pub color: !,
    pub footericon: !,
    pub title: !,
    pub thumburl: !,
    pub titlelink: !,
    pub pretext: !,
    pub footer: !,
    pub authorname: !,
    pub text: !,
    pub authorlink: !,
    pub authoricon: !,
}
