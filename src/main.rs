mod deserialize_callback;
use actix_web::{
    error,
    web::Json,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use clap::Parser;
use deserialize_callback::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, env)]
    vktoken: String,
    #[clap(long, env)]
    vkcommunityid: u32,
}

async fn index(req: Json<RequestData>, state: Data<AppState>) -> impl Responder {
    match req.into_inner() {
        RequestData::Confirmation(val) => {
            dbg!("Respond confirmation", val);
            HttpResponse::Ok().body(state.vktoken.clone())
        }
        RequestData::WallPostNew(val) => {
            dbg!("Respond message", &val);
            HttpResponse::Ok().json(val)
        }
        _ => HttpResponse::Ok().body("ok"),
    }
}

#[derive(Debug, Clone)]
struct AppState {
    vktoken: String,
    vkcommunityid: u32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let vktoken = Data::new(AppState {
        vktoken: cli.vktoken,
        vkcommunityid: cli.vkcommunityid,
    });

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(json_config)
                .app_data(vktoken.clone())
                .route(web::post().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
