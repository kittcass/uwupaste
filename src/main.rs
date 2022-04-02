#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

pub mod api;
pub mod db;

#[database("uwupaste")]
pub struct UwuPasteDbConn(diesel::PgConnection);

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", api::routes())
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(UwuPasteDbConn::fairing())
}
