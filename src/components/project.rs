use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Projects(pub Vec<ProjectItem>);

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    pub name: String,
    pub description: Option<String>,
    pub date_range: String,
    pub website: Option<String>,
    pub sumary: Option<String>,
}
