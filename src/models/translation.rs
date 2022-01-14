use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum TranslationMode {
    Shakespeare,
    Yoda,
}

#[derive(Serialize, Debug)]
pub struct TranslationRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationContents {
    pub translated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranslationResponse {
    pub contents: TranslationContents,
}
