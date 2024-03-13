use crate::service::service::get_service;
use axum::extract::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BodyParams {
    pub name: String,
    pub email: String,
}
pub async fn get_handler(Json(body): Json<BodyParams>) -> String {
    get_service(body).await;

    String::from("query complete")
}
