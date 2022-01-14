use crate::error::error_chain_fmt;
use crate::funtranslation_client::FuntranslationClient;
use crate::models::translation::*;
use crate::pokemon_client::PokemonClient;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error)]
pub enum PokemonError {
    #[error(transparent)]
    NotFoundError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PokemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PokemonError {
    fn status_code(&self) -> StatusCode {
        match self {
            PokemonError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonName {
    pub name: String,
}

///GET /pokemon/{name}
pub async fn pokemon_information(
    pokemon_client: web::Data<PokemonClient>,
    path_name: web::Path<PokemonName>,
) -> Result<HttpResponse, PokemonError> {
    let pokemon = pokemon_client
        .get_pokemon_information(path_name.into_inner())
        .await
        .context("Failed to obtain pokemon information.")?;

    Ok(HttpResponse::Ok().json(pokemon))
}

///GET /pokemon/translated/{name}
pub async fn pokemon_translated(
    pokemon_client: web::Data<PokemonClient>,
    funtranslation_client: web::Data<FuntranslationClient>,
    path_name: web::Path<PokemonName>,
) -> Result<HttpResponse, PokemonError> {
    let mut pokemon = pokemon_client
        .get_pokemon_information(path_name.into_inner())
        .await
        .context("Failed to obtain pokemon information.")?;

    let translation_mode = if pokemon.is_cave() || pokemon.legendary {
        TranslationMode::Yoda
    } else {
        TranslationMode::Shakespeare
    };

    let translation_request = TranslationRequest {
        text: pokemon.description.flavor_text.clone(),
    };

    let translated = funtranslation_client
        .post_translate(translation_request, translation_mode)
        .await
        .context("Failed to obtain translation");

    //if we get a response, modify description
    if let Ok(response) = translated {
        pokemon.description.flavor_text = response.contents.translated;
    }

    Ok(HttpResponse::Ok().json(pokemon))
}
