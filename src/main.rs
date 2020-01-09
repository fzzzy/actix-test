use actix_web::{web, App, Error, HttpResponse, HttpServer};

use futures::future::{ok, Future};

fn handle_request() -> impl Future<Output = Result<HttpResponse, Error>> {
    ok(HttpResponse::Ok().body("hello"))
}

async fn handle_request_async_await() -> HttpResponse {
    HttpResponse::Ok().body("olleh")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(handle_request)))
            .service(web::resource("/async").route(web::get().to(handle_request_async_await)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
