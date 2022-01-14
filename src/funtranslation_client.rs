use crate::models::translation::*;
use reqwest::{Client, Url};

pub struct FuntranslationClient {
    http_client: Client,
    base_url: Url,
}

impl FuntranslationClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_millis(10000))
                .build()
                .unwrap(),
            base_url: Url::parse(&base_url).unwrap(),
        }
    }

    pub async fn post_translate(
        &self,
        translation_request: TranslationRequest,
        translation_mode: TranslationMode,
    ) -> Result<TranslationResponse, reqwest::Error> {
        let url = Url::join(&self.base_url, &translation_mode.to_string())
            .expect("Unable to join base URL with pokemon name");

        let translated = self
            .http_client
            .post(&url.to_string())
            .json(&translation_request)
            .send()
            .await?
            .json::<TranslationResponse>()
            .await?;

        Ok(translated)
    }
}
