#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, HttpResponse, Error};

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/plain").body("Hello World") )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
          .service(web::resource("/").to(index))
    })
      .bind("localhost:9999")
      .expect("Can not bind to port 9999")
      .run()
      .await
}
