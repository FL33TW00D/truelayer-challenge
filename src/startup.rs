use crate::{
    api::*, configuration::*, funtranslation_client::FuntranslationClient,
    pokemon_client::PokemonClient,
};
use actix_web::{dev::Server, middleware::Logger, web, web::Data, App, HttpServer};
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let pokemon_client = PokemonClient::new(configuration.pokemon_client.base_url);
        let funtranslation_client =
            FuntranslationClient::new(configuration.funtranslation_client.base_url);

        let port = configuration.application.port;
        let server = run(
            pokemon_client,
            funtranslation_client,
            configuration.application,
            address,
        )?;
        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub fn run(
    pokemon_client: PokemonClient,
    funtranslation_client: FuntranslationClient,
    application_settings: ApplicationSettings,
    app_url: String,
) -> Result<Server, std::io::Error> {
    let _ = env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("info"));

    let application_settings = Data::new(application_settings);
    let pokemon_client = Data::new(pokemon_client);
    let funtranslation_client = Data::new(funtranslation_client);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(application_settings.clone())
            .app_data(pokemon_client.clone())
            .app_data(funtranslation_client.clone())
            .service(web::resource("/health_check").route(web::get().to(health_check)))
            .service(web::resource("/pokemon/{name}").route(web::get().to(pokemon_information)))
            .service(
                web::resource("/pokemon/translated/{name}")
                    .route(web::get().to(pokemon_translated)),
            )
            .wrap(Logger::default())
    })
    .bind(&app_url)?
    .run();
    Ok(server)
}
