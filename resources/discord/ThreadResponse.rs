#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ThreadResponse {
    pub ownerid: !,
    pub type: !,
    pub guildid: !,
    pub flags: !,
    pub id: !,
    pub bitrate: !,
    pub messagecount: !,
    pub totalmessagesent: !,
    pub name: !,
    pub lastpintimestamp: !,
    pub rtcregion: !,
    pub permissions: !,
    pub ratelimitperuser: !,
    pub membercount: !,
    pub userlimit: !,
}
