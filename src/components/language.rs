use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Languages(pub Vec<LanguageItem>);

#[derive(Serialize, Deserialize)]
pub struct LanguageItem {
    pub name: String,
    pub description: Option<String>,
    pub level: Option<u8>,
}
