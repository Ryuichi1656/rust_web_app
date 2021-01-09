
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum CustomError {}

impl ResponseError for CustomError{}

#[get("/")]
async fn index() -> Result<HttpResponse, CustomError> {
    let response_body = "Hello world!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8088")?
        .run()
        .await?;
    Ok(())
}
