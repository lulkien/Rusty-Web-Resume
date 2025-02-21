use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Summary(pub String);
