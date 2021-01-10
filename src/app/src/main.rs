
use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template;

struct TodoEntry {
    id: u32,
    text: String
}

// Template
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>
}

// エラー用
#[derive(Error, Debug)]
enum CustomError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for CustomError{}

// Topページ
#[get("/")]
async fn index() -> Result<HttpResponse, CustomError> {
    // let response_body = "Hello world!";
    // Ok(HttpResponse::Ok().body(response_body))
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string()
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string()
    });
    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body)
    )
}

// サーバー起動スクリプト
#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8088")?
        .run()
        .await?;
    Ok(())
}
