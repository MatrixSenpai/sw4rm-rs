#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct InviteApplicationResponse {
    pub maxparticipants: !,
    pub name: !,
    pub custominstallurl: !,
    pub icon: !,
    pub verifykey: !,
    pub description: !,
    pub flags: !,
    pub id: !,
    pub coverimage: !,
    pub slug: !,
    pub botpublic: !,
    pub botrequirecodegrant: !,
    pub privacypolicyurl: !,
    pub termsofserviceurl: !,
}
