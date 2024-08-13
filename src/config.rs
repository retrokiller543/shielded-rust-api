use actix_web::{get, Responder, HttpResponse, web};
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use shielded_rust::config::{Config, ConfigBuilder};
use crate::error::BackendError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigRequest {
    get_default: bool,
    get_base64: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigResponse {
    pub config: Config
}

#[get("/api/config")]
pub async fn config(data: web::Query<ConfigRequest>) -> Result<impl Responder, BackendError> {
    trace!("config: {:?}", data);

    let config = if data.get_default {
        Config::default()
    } else if data.get_base64 {
        ConfigBuilder::default().with_base64().build()
    } else {
        Config::default()
    };

    debug!("config: {:?}", config);

    Ok(HttpResponse::Ok().json(ConfigResponse { config }))
}