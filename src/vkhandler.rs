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
    endpoint: Option<VkEndpointItems>,
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
            endpoint: Some(endpoint.clone()),
        }
    } else if let Some(endpoint) = endpoint.check(uuid) {
        Endpoint {
            status: EndpointStatus::Known,
            endpoint: Some(endpoint.clone()),
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

        // @TODO
        if let EndpointStatus::Waiting = current.status {
            println!("Точка ещё не подтверждена тут функционал подтверждения")
        }

        return HttpResponse::Ok().body(endpoint.vk_conrifmation_token.clone());
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
        let response = confirmation(
            uuid.into_inner(),
            channel_id,
            req_secret,
            state.endpoints,
            state.waiting_confirmation_endpoints,
        ).await;

        return response;
    }

    return HttpResponse::Ok().body("ok")
}
