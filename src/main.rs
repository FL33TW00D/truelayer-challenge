use truelayer::configuration::get_configuration;
use truelayer::startup::Application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get server configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
