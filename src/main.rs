use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

macro_rules! compact {
    ($content:ident, $($x:ident),+) => {
        $(
          let re_string = format!(r"\{{\{{\s*\${}\s*\}}\}}", stringify!($x));
          let re = Regex::new(&re_string).unwrap();
          $content = re.replace_all(&$content, $x).into_owned();
        )*
    };
}

fn view(file_path: &str) -> String {
    fs::read_to_string(format!("./view/{}", file_path))
        .expect("Something went wrong reading the file")
}

trait View {
    fn return_view(&self) -> HttpResponse;
}

impl View for String {
    fn return_view(&self) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(self.to_string())
    }
}

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
    let data = serde_json::to_string(&data).unwrap();
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
