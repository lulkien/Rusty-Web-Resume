use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Skills(pub Vec<SkillItem>);

#[derive(Serialize, Deserialize)]
pub struct SkillItem {
    pub name: String,
    pub description: Option<String>,
    pub level: Option<u8>,
}
