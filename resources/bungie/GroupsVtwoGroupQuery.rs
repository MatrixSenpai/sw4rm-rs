#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupsVtwoGroupQuery {
    pub items_per_page: !,
    pub locale_filter: !,
    pub sort_by: !,
    pub name: !,
    pub creation_date: !,
    pub tag_text: !,
    pub current_page: !,
    pub request_continuation_token: !,
    pub group_member_count_filter: !,
    pub group_type: !,
}
