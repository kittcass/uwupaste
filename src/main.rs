#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;

#[database("uwupaste")]
struct UwuPasteDbConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[get("/<identifier>")]
fn get(identifier: String) {
    // TODO
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(UwuPasteDbConn::fairing())
}
