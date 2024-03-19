#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigUserTheme {
    pub user_theme_name: !,
    pub user_theme_description: !,
    pub user_theme_id: !,
}
