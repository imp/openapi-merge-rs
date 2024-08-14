use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub append: bool,
    pub title: Option<Title>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub heading_level: Option<u8>,
    pub value: String,
}
