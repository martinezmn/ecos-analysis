#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use serde::json::{Json, Value, json};
use serde::{Serialize, Deserialize};

mod teste;

// #[get("/<id>/<name>")]
// fn project() -> String {
//     return teste::print_hello()
// }

// #[get("/projects")]
// fn index() -> content::Json<&'static str> {
//     content::Json("{ hi: \"world\" }")
// }

#[post("/", format = "json", data = "<message>")]
async fn new(message: Json<Message<'_>>, list: Messages<'_>) -> Value {
    let mut list = list.lock().await;
    let id = list.len();
    list.push(message.message.to_string());
    json!({ "status": "ok", "id": id })
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
