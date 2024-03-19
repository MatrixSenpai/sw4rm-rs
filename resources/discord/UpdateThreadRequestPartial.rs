#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateThreadRequestPartial {
    pub userlimit: !,
    pub rtcregion: !,
    pub invitable: !,
    pub archived: !,
    pub bitrate: !,
    pub name: !,
    pub locked: !,
    pub ratelimitperuser: !,
    pub flags: !,
}
