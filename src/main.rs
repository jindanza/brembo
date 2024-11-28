use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

static mut ITEMS: Vec<Item> = vec![];

#[get("/items")]
async fn get_items() -> impl Responder {
    unsafe { web::Json(&ITEMS) }
}

#[post("/items")]
async fn create_items(item: web::Json<Item>) -> impl Responder {
    unsafe {
        ITEMS.push(item.into_inner());
        "Item Added"
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_items).service(create_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
