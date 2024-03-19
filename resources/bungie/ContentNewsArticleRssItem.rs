#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ContentNewsArticleRssItem {
    pub html_content: !,
    pub optional_mobile_image_path: !,
    pub title: !,
    pub unique_identifier: !,
    pub link: !,
    pub description: !,
    pub pub_date: !,
    pub image_path: !,
}
