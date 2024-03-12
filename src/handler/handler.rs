use crate::service::service::get_service;

pub async fn get_handler() -> String {
    get_service().await
}
