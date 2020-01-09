use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};

use futures::Future;

pub fn handle_request() -> impl Future<Item = HttpResponse, Error = Error> {
    futures::future::ok(HttpResponse::Ok().body("hello"))
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::get().to_async(handle_request)))
    })
    .bind("127.0.0.1:3000")?
    .run()
}
