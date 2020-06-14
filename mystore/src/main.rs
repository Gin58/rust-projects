pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(|| {
        App::new()
          .service(web::resource("/products").to(handlers::products::index))
    })
      .bind("localhost:9999")?
      .run()
      .await?;
    sys.await?;
    Ok(server_res)
}
