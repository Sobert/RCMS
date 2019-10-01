#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Serialize;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[derive(Serialize)]
struct Article {
    id: i32,
    date: String,
    title: String,
    content: String,
    author: Author,
    published: bool
}

#[derive(Serialize)]
struct Author {
    name: String
}