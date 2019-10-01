#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use serde::Serialize;

use rocket_contrib::templates::Template;
use tera::Context;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn get_article(name: &RawStr) -> Template {
    let article = Article {
        date: "10/10/2020".to_string(),
        title: "Test title".to_string(),
        content: "lorem ipsum bla bla".to_string(),
        author: Author { name: "Basic Author".to_string() },
        published: true, 
    };
    let mut context = Context::new();
    context.insert("article", &article);
    Template::render("article", context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, get_article])
    .attach(Template::fairing())
    .launch();
}

#[derive(Serialize)]
struct Article {
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