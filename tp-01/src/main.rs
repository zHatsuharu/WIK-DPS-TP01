use std::collections::HashMap;
use actix_web::{get, App, HttpResponse, HttpServer, http::header::{HeaderMap, HeaderValue, self}, web, HttpRequest, Responder};
use serde_json::{Result, Value};

#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(convert(req.headers()))
}

fn get_host<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("host")?.to_str().ok()
}

fn convert(headers: &HeaderMap) -> HashMap<String, Vec<String>> {
    let mut header_hashmap = HashMap::new();
    for (k, v) in headers {
        let k = k.as_str().to_owned();
        let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
        header_hashmap.entry(k).or_insert_with(Vec::new).push(v)
    }
    header_hashmap
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}