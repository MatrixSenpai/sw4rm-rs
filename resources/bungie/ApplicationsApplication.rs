#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApplicationsApplication {
    pub origin: !,
    pub link: !,
    pub status: !,
    pub redirect_url: !,
    pub creation_date: !,
    pub application_id: !,
    pub status_changed: !,
    pub first_published: !,
    pub scope: !,
    pub name: !,
    pub override_authorize_view_name: !,
}
