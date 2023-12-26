mod deserialize_callback;
use actix_web::{error, web, web::Json, App, HttpResponse, HttpServer, Responder};
use deserialize_callback::*;

use clap::Parser;

const TOKEN: &'static str =
    "c0223f775444cf3d58a8a1442ec76a9571c8f58e3e24616d9440f73dc43022bbead9b2e576cb41d09c0a1";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, env)]
    vk_token: String,
}
async fn index(data: Json<Data>, token: &str) -> impl Responder {
    println!("token {}", token);
    let data: Data = data.into_inner();
    match data {
        Data::Confirmation(val) => {
            dbg!("Respond confirmation", val);
            HttpResponse::Ok().body(TOKEN)
        }
        Data::MessageNew(val) => {
            dbg!("Respond message", &val);
            HttpResponse::Ok().json(val)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    HttpServer::new(move || {

        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        let vktoken = cli.vk_token.as_str();
        //
        let index = web::post().to(move |req| index(req, vktoken));

        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(json_config)
                .route(index),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
