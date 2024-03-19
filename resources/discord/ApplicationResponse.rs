#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationResponse {
    pub name: !,
    pub privacypolicyurl: !,
    pub id: !,
    pub description: !,
    pub icon: !,
    pub custominstallurl: !,
    pub maxparticipants: !,
    pub verifykey: !,
    pub coverimage: !,
    pub botpublic: !,
    pub slug: !,
    pub botrequirecodegrant: !,
    pub flags: !,
    pub termsofserviceurl: !,
}
