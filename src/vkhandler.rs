use crate::{deserialize_callback::RequestData, endpoints::VkEndpointItems};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Responder,
};
use uuid::Uuid;

use crate::{
    deserialize_callback::RequestVk,
    endpoints::{self, VkEndpoints},
    WebState,
};

enum EndpointStatus {
    Waiting,
    Known,
    Unknown,
}

struct Endpoint {
    status: EndpointStatus,
    endpoint: Option<&'static VkEndpointItems>,
}

pub async fn confirmation(
    uuid: Uuid,
    vk_group_id: u32,
    secret: String,
    endpoint: VkEndpoints,
    waiting: VkEndpoints,
) -> impl Responder {
    let current = if let Some(endpoint) = waiting.check(uuid) {
        Endpoint {
            status: EndpointStatus::Waiting,
            endpoint: Some(endpoint),
        }
    } else if let Some(endpoint) = endpoint.check(uuid) {
        Endpoint {
            status: EndpointStatus::Known,
            endpoint: Some(endpoint),
        }
    } else {
        Endpoint {
            status: EndpointStatus::Unknown,
            endpoint: None,
        }
    };

    if let Some(endpoint) = current.endpoint {
        if !endpoint.verify_secret(secret) {
            return HttpResponse::Forbidden().body("secret don`t match");
        }

        if let EndpointStatus::Waiting = current.status {

            println!("Точка ещё не подтверждена тут функционал подтверждения")
        }        
    }

    HttpResponse::NotFound().body("Not found")
}

pub async fn handle_callback(
    uuid: web::Path<Uuid>,
    state: Data<WebState>,
    req: Json<RequestVk>,
) -> impl Responder {
    let RequestVk {
        channel_id,
        secret: req_secret,
        data,
    } = req.into_inner();

    if let RequestData::Confirmation = data {
        return HttpResponse::Ok().body(state.vk_confirmation_token.clone());
    }

    HttpResponse::Ok().body("ok")
}
