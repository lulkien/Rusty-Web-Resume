use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PersonalInfo {
    pub fullname: String,
    pub title: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub localtion: Option<String>,
    pub custom_field: Option<Vec<CustomPersonalInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomPersonalInfo {
    pub name: String,
    pub data: String,
}
