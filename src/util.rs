use serde::{Deserialize, Serialize};

/// String that can be displayed in multiple languages
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatedString {
    pub en: String,
    pub nl: String,
}
