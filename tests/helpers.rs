use std::net::TcpListener;
use truelayer::configuration::get_configuration;
use truelayer::startup::Application;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub pokemon_server: MockServer,
    pub translation_server: MockServer,
    pub api_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let pokemon_server = MockServer::start().await;
        let translation_server = MockServer::start().await;

        let configuration = {
            let mut c = get_configuration().expect("Failed to read configuration.");
            let listener =
                TcpListener::bind("127.0.0.1:0").expect("Unabled to obtain local address.");
            let local_addr = listener
                .local_addr()
                .expect("Unabled to obtain local address");
            c.application.port = local_addr.port();
            c.pokemon_client.base_url = pokemon_server.uri();
            c.funtranslation_client.base_url = translation_server.uri();
            c
        };

        let application = Application::build(configuration.clone())
            .await
            .expect("Failed to build application.");

        let _ = tokio::spawn(application.run_until_stopped());

        let api_client = reqwest::Client::builder()
            .build()
            .expect("Unabled to construct reqwest client");

        Self {
            address: format!("http://localhost:{}", configuration.application.port),
            port: configuration.application.port,
            pokemon_server,
            translation_server,
            api_client,
        }
    }

    pub async fn get_pokemon_information(self, name: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/pokemon/{}", &self.address, &name))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_pokemon_translation(self, name: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}/pokemon/translated/{}", &self.address, &name))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
