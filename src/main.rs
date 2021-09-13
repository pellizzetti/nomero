#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Message {
    result: usize
}

fn solver(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let old = a;
        a = b;
        b += old;
    }

    b
}

#[get("/fibonacci/<number>")]
fn fibonacci(number: usize) -> Json<Message> {
    let result: usize = solver(number);
    Json(Message {
        result
    })
}

#[get("/")]
fn index() -> JsonValue {
    json!({
        "name": "Nomero",
        "description": "No-code, composable, enterprise-grade fibonacci solver backed by blockchain, quantum computing and AI."
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

rocket_healthz::healthz!();

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, healthz])
        .mount("/api", routes![fibonacci])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}