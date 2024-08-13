use actix_web::{post, Responder, HttpResponse, web};
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use shielded_rust::config::Config;
use shielded_rust::symmetric::caesar;
use crate::error::BackendError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaesarRequest {
    pub input: String,
    pub key: usize,
    pub config: Config
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaesarResponse {
    pub output: String
}

#[post("/api/caesar/encrypt")]
pub async fn caesar_encrypt(data: web::Json<CaesarRequest>) -> Result<impl Responder, BackendError> {
    debug!("caesar_encrypt: {:?}", data);
    let encrypted = caesar::encrypt_string(&data.input, &data.config, data.key).map_err(|e| BackendError::from(e))?;
    debug!("caesar_encrypt: {:?}", encrypted);
    Ok(HttpResponse::Ok().json(CaesarResponse { output: encrypted }))
}

#[post("/api/caesar/decrypt")]
pub async fn caesar_decrypt(data: web::Json<CaesarRequest>) -> Result<impl Responder, BackendError> {
    debug!("caesar_decrypt: {:?}", data);
    let decrypted = caesar::decrypt_string(&data.input, &data.config, data.key).map_err(|e| BackendError::from(e))?;
    debug!("caesar_decrypt: {:?}", decrypted);
    Ok(HttpResponse::Ok().json(CaesarResponse { output: decrypted }))
}