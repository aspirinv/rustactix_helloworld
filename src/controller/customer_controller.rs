use actix_web::{Result, get, web};
use actix_web::web::Json;

use crate::domain::customer::Customer;

#[get("customer/{id}/{name}")]
pub async fn get_customer(info: web::Path<(u32, String)>) -> Result<Json<Customer>> {
    Ok(web::Json(Customer {
        name: info.1.to_string(),
        id: info.0
    }))
}