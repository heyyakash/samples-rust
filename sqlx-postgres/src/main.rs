mod handlers;
mod model;
mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be present in env");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("Connection to DB successfull!");
                pool
            }
            Err(_pool) => {
                println!("Connection to DB unsuccessfull");
                std::process::exit(1);
            }
         };

    println!("Server Started!");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{db:pool.clone()}))
            .configure(handlers::Routerconfig)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

