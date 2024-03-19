#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreatedThreadResponse {
    pub id: !,
    pub permissions: !,
    pub ownerid: !,
    pub messagecount: !,
    pub type: !,
    pub lastpintimestamp: !,
    pub rtcregion: !,
    pub name: !,
    pub totalmessagesent: !,
    pub bitrate: !,
    pub membercount: !,
    pub ratelimitperuser: !,
    pub flags: !,
    pub userlimit: !,
    pub guildid: !,
}
