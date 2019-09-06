use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::Deserialize;
use serde_derive::Serialize;
use whatlang::detect;

#[derive(Debug, Deserialize)]
struct Langs {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LanfInfo {
    lang: String,
    script: String,
    confidence: f64,
    is_reliable: bool,
}

//type LanfInfo = whatlang::Info;

impl Responder for LanfInfo {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self)?;

        //Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/tester", web::post().to(tester))
            .route("/", web::post().to(lang_guesser))
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to port 8080")
    .run()
    .unwrap();
}

fn lang_guesser(evt: web::Json<Langs>) -> impl Responder {
    //fn lang_guesser() -> impl Responder {
    //println!("{:?}", evt);
    //"Hello !"
    let info = detect(&evt.text).unwrap();
    LanfInfo {
        lang: info.lang().to_string(),
        script: info.script().to_string(),
        confidence: info.confidence(),
        is_reliable: info.is_reliable(),
    }
}

fn tester(req: HttpRequest) -> String {
    println!("{:?}", req.peer_addr());
    println!("{:?}", req.connection_info());
    println!("{:?}", req.head());
    println!("{:?}", req.version());
    "test".to_string()
}

fn hello() -> impl Responder {
    "Hello World!"
}
