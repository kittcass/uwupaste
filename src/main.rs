#![feature(try_trait_v2)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use db::UwuPasteDbConn;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod api;
pub mod frontend;
pub mod db;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", frontend::routes())
        .mount("/api", api::routes())
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(UwuPasteDbConn::fairing())
}
