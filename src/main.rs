#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs;
use std::path::{Path, PathBuf};
use rocket::http::RawStr;
use rocket::response::NamedFile;
use serde::{Serialize, Deserialize};

use rocket_contrib::templates::Template;
use tera::Context;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("articles", &get_articles_index());
    Template::render("index", context)
}

#[get("/<name>")]
fn get_article(name: &RawStr) -> Template {
    let article = parse_article(read_file(name.to_string()));
    let mut context = Context::new();
    context.insert("article", &article);
    Template::render("article", context)
}

#[get("/<path..>")]
fn assets(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(path)).ok()
}

fn read_file(name: String) -> String {
    let path = Path::new("articles").join(format!("{}.json",name));
    println!("path: {:?}", path);
    let content = fs::read_to_string(path);
    match content {
        Err(e) => panic!("error: {}", e),
        Ok(content) => {
            println!("character read: {}", content.len());
            content
        }
    }
}

fn parse_article(content: String) -> Article {
    match serde_json::from_str::<Article>(&content) {
        Err(e) => panic!("error in parsing article: {}", e),
        Ok(v) => v
    }
}

fn get_articles_index() -> Vec<ArticleExcerpt> {
    match serde_json::from_str::<Vec<ArticleExcerpt>>(&read_file("index".to_string())) {
        Err(e) => panic!("Error retrieving index: {}", e),
        Ok(v) => v
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, get_article])
    .mount("/public", routes![assets])
    .attach(Template::fairing())
    .launch();
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    date: String,
    title: String,
    intro: String,
    content: String,
    author: Author,
    published: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct ArticleExcerpt {
    date: String,
    title: String,
    intro: String,
    link: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: String
}