mod endpoints;
use std::sync::{Arc, Mutex};

use endpoints::VkEndpoints;
mod bot;
mod vkhandler;

use teloxide::Bot;
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

use crate::config::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug)]
pub struct WebState {
    bot: Bot,
    endpoints: Mutex<VkEndpoints>,
    waiting_confirmation_endpoints: Arc<Mutex<VkEndpoints>>,
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
    let AppState { host, port } = config::read_config();

    let endpoints = VkEndpoints::read("endpoints.yml".to_string())
        .await
        .expect("Не смог прочитать точки файл конфигурации");
    let endpoints = Mutex::new(endpoints);

    let waiting_confirmation_endpoints =
        VkEndpoints::read("waiting.yml".to_string()).await.unwrap();

    let waiting_confirmation_endpoints = Arc::new(Mutex::new(waiting_confirmation_endpoints));

    dbg!(&waiting_confirmation_endpoints);
    let arc = Arc::clone(&waiting_confirmation_endpoints);
    let bot = bot::create();

    let state = Data::new(WebState {
        endpoints,
        waiting_confirmation_endpoints,
        bot: bot.clone(),
    });

    let json_config = configure_json();

    actix_web::rt::spawn(async move { bot::dispatch(bot.clone(), arc).await });

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
