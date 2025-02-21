use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Interests(pub Vec<String>);
