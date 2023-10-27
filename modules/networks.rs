use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use std::sync::Mutex;

async fn print_me() -> impl Responder {
    std::println!("Hello");
    HttpResponse::Ok().body("OK")
}

async fn index(tmpl: web::Data<Mutex<Tera>>) -> impl Responder {
    let tera = tmpl.lock().unwrap();
    // Şablon için verileri belirleyin
    let mut context = Context::new();
    context.insert("page_title", "Merhaba Dünya");
    context.insert("heading", "Salam azerbaycan xalqi");
    context.insert("content", "Bu test kodudur");

    let rendered = tera.render("main.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}

#[actix_rt::main]
pub async fn initServer(serv: &str, port: &str) -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    let tera = web::Data::new(Mutex::new(tera));

    HttpServer::new(move || { // 'move' anahtar kelimesi ekleniyor
        App::new()
            .app_data(tera.clone())  // Data extractor'ü yapılandırın
            .route("/", web::get().to(index))
            .route("/print", web::get().to(print_me))

    })
    .bind(format!("{}:{}", serv, port))?
    .run()
    .await
}

