use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Pool, Client};
use crate::models::Status;
use crate::db;


pub async fn status() -> impl Responder {
    web::Json(Status{status : "OK".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error get pool");
    
    let result = db::get_todo(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}