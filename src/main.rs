use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

mod view;
use view::{json, view, View};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u32,
    name: String,
}

#[get("hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let name = name.to_string();
    let title = "Laravel Rust";
    let data = [
        Todo {
            id: 1,
            name: "Rust".to_string(),
        },
        Todo {
            id: 2,
            name: "Go".to_string(),
        },
    ];
    let data = json(&data).unwrap();
    let mut view = view("index.html");
    compact!(view, name, title, data);
    view.return_view()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(Files::new("/public", "./public"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
