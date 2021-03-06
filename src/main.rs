mod models;
mod config;
mod handler;
mod db;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;

use crate::handler::*;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
     
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("server up at http://{}:{}/ ", config.server.host, config.server.port);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status)) 
            .route("/todos{_:/?}", web::get().to(get_todos))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
