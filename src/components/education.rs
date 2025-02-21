use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Education(pub Vec<EducationItem>);

#[derive(Serialize, Deserialize)]
pub struct EducationItem {
    pub institution: String,
    pub degree: Option<String>,
    pub area_of_study: Option<String>,
    pub date_range: String,
    pub score: Option<f32>,
    pub website: Option<String>,
    pub sumary: Option<String>,
}
