mod endpoints;
use endpoints::VkEndpoints;
mod bot;
mod vkhandler;

use teloxide::Bot;
use uuid::uuid;
pub mod attachments;
pub mod config;
pub mod deserialize_callback;

use actix_web::{
    error, get,
    web::{self, Data, JsonConfig},
    App, HttpResponse, HttpServer, Responder,
};

#[cfg(feature = "prometheus")]
use actix_web_prom::PrometheusMetricsBuilder;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug)]
pub struct WebState {
    bot: Bot,
    endpoints: VkEndpoints,
    waiting_confirmation_endpoints: VkEndpoints,
}

fn configure_json() -> JsonConfig {
    web::JsonConfig::default().error_handler(|err, _req| {
        dbg!(&err);
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // @TODO thread spawn?
    let (host, port, state) = config::read_config();
    let host = host.unwrap_or("0.0.0.0".to_string());
    let port = port.unwrap_or(3000);

    let bot = bot::create().await;

    let state = Data::new(WebState {
        endpoints: VkEndpoints::new(
            state.vk_confirmation_token.clone(),
            state.vk_secret.clone(),
            state.telegram_group_id.clone(),
            uuid!("44663e93-c1c2-4ea4-95b6-d957632c408f"),
        ),
        waiting_confirmation_endpoints: VkEndpoints::new(
            state.vk_confirmation_token,
            state.vk_secret,
            state.telegram_group_id,
            uuid!("987ec6cd-6275-4151-b80a-b8f7f13e6357"),
        ),
        bot,
    });

    let json_config = configure_json();

    #[cfg(feature = "prometheus")]
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        #[cfg(not(feature = "prometheus"))]
        let app = App::new();

        #[cfg(feature = "prometheus")]
        let app = App::new().wrap(prometheus.clone());

        app.service(hello).service(
            web::resource("/{uid}")
                .app_data(json_config.clone())
                .app_data(state.clone())
                .route(web::post().to(vkhandler::handle_callback)),
        )
    })
    .bind((host, port))?
    .run()
    .await
}
