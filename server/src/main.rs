extern crate actix_web;

use actix_web::{fs, http, server, App, HttpRequest};
use std::io;

fn main() -> io::Result<()> {
    server::new(|| {
        App::new()
            .route("/", http::Method::GET, |_: HttpRequest| {
                fs::NamedFile::open("index.html")
            })
            //.route("/index", http::Method::GET, |_: HttpRequest| {
            //    fs::NamedFile::open("index.js")
            //})
            //.route("/webgl_test", http::Method::GET, |_: HttpRequest| {
            //    fs::NamedFile::open("webgl_test.js")
            //})
            //.route("/webgl_test_bg", http::Method::GET, |_: HttpRequest| {
            //    fs::NamedFile::open("webgl_test_bg.wasm")
            //})
            .handler("/", fs::StaticFiles::new(".").unwrap())
    }).bind("0.0.0.0:11484")?
        .run();

    Ok(())
}
