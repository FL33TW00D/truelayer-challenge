use crate::api::PokemonName;
use crate::models::Pokemon;
use reqwest::{Client, Url};

pub struct PokemonClient {
    http_client: Client,
    base_url: Url,
}

impl PokemonClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::builder()
                .timeout(std::time::Duration::from_millis(10000))
                .build()
                .unwrap(),
            base_url: Url::parse(&base_url).unwrap(),
        }
    }

    pub async fn get_pokemon_information(
        &self,
        poke_name: PokemonName,
    ) -> Result<Pokemon, reqwest::Error> {
        let url = Url::join(&self.base_url, &poke_name.name)
            .expect("Unable to join base URL with pokemon name");

        let poke_info = self
            .http_client
            .get(&url.to_string())
            .send()
            .await?
            .json::<Pokemon>()
            .await?;

        Ok(poke_info)
    }
}
