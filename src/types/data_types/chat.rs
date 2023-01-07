use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub username: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Chat {
    pub fn new(
        id: i64,
        first_name: impl Into<String>,
        username: impl Into<String>,
        type_field: impl Into<String>,
    ) -> Self {
        Chat {
            id,
            first_name: first_name.into(),
            username: username.into(),
            type_field: type_field.into(),
        }
    }
}
