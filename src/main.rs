#![allow(non_snake_case)]

// Importation des fonctions
mod Euclide;
mod Ferma;
mod Palindrome;
mod Conversion;
mod Fibonacci;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Utilisation des fonctions
    let mot: &str = "TAT";
    Palindrome::palin(mot);

    let a = 24;
    let b = 36;
    Euclide::euclide(a, b);

    let a=13;
    Ferma::fermat(a);

    let nb = 25;
    Conversion::conversion(nb);

    let n = 10;
    Fibonacci::fibo(n);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}