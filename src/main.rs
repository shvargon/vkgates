pub mod attachments;
pub mod config;
pub mod deserialize_callback;

mod vkhandler;

use actix_web::{
    error, get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // @TODO thread spawn?
    let state = Data::new(config::read_config());
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        dbg!(&err);
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
    });

    HttpServer::new(move || {
        App::new().service(hello).service(
            web::resource("/")
                .app_data(json_config.clone())
                .app_data(state.clone())
                .route(web::post().to(vkhandler::index)),
        )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
