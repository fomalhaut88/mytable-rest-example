#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};

use mytable::TableTrait;

mod db;
mod person;

use person::Person;


#[get("/version")]
fn version() -> JsonValue {
    json!({"version": env!("CARGO_PKG_VERSION")})
}


#[get("/get/<id>")]
fn get(id: usize,
            conn: &db::MytableConnection
        ) -> Result<Json<JsonValue>, Status> {
    let table = conn.lock_table();
    match Person::get(&table, id) {
        Ok(person) => Ok(Json(person.as_json())),
        Err(_) => Err(Status::NotFound)
    }
}


#[post("/insert", format = "application/json", data = "<input>")]
fn insert(
            input: Json<JsonValue>,
            conn: &db::MytableConnection
        ) -> Result<Json<JsonValue>, Status> {
    let mut person = Person::new(
        input["name"].to_string().as_str(),
        input["age"].as_u64().unwrap() as u32
    );
    let table = conn.lock_table();
    match person.insert(&table) {
        Ok(_) => Ok(Json(person.as_json())),
        Err(_) => Err(Status::InternalServerError)
    }
}


#[post("/update", format = "application/json", data = "<input>")]
fn update(
            input: Json<JsonValue>,
            conn: &db::MytableConnection
        ) -> Result<Json<JsonValue>, Status> {
    let person = Person::from_json(&input);
    let table = conn.lock_table();
    match person.update(&table) {
        Ok(_) => Ok(Json(person.as_json())),
        Err(_) => Err(Status::InternalServerError),
    }
}


fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![version, get, insert, update])
        .launch();
}



