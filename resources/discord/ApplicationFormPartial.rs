#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationFormPartial {
    pub icon: !,
    pub custominstallurl: !,
    pub maxparticipants: !,
    pub interactionsendpointurl: !,
    pub roleconnectionsverificationurl: !,
    pub coverimage: !,
    pub flags: !,
}
