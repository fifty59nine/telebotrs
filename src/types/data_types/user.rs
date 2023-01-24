use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
    pub language_code: String,
}

impl User {
    pub fn new(
        id: i64,
        is_bot: bool,
        first_name: impl Into<String>,
        username: impl Into<String>,
        language_code: impl Into<String>,
    ) -> Self {
        User {
            id,
            is_bot,
            first_name: first_name.into(),
            username: username.into(),
            language_code: language_code.into(),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(0, false, "Default", "default", "UA")
    }
}
