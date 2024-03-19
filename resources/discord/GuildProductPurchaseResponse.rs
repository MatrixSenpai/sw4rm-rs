#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GuildProductPurchaseResponse {
    pub productname: !,
    pub listingid: !,
}
