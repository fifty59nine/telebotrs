use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub username: String,
    #[serde(rename="type")]
    pub type_field: String,
}


