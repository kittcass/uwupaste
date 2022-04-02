#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use uuid::Uuid;

pub mod db;

use db::{insert_paste, retrieve_paste, NewPaste, PasteType};

#[database("uwupaste")]
struct UwuPasteDbConn(diesel::PgConnection);

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[post("/", data = "<value>")]
async fn post_paste(value: String, conn: UwuPasteDbConn) -> String {
    // TODO error handling

    let paste = NewPaste::new(PasteType::Text, value).build();
    let id = paste.id;
    conn.run(move |conn| insert_paste(&paste, &conn))
        .await
        .expect("could not insert paste");
    id.to_string()
}

#[get("/<id>")]
async fn get_paste(id: &str, conn: UwuPasteDbConn) -> Option<String> {
    // TODO error handling

    let id = Uuid::parse_str(id).unwrap(); // TODO
    conn.run(move |conn| retrieve_paste(&id, &conn))
        .await
        .ok()
        .map(|p| p.value)
}

// #[delete("/<id>?<delete_code>")]
// fn delete_paste(id: &str, delete_code: &str) {
//     // TODO
// }

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, post_paste, get_paste])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
        .attach(UwuPasteDbConn::fairing())
}
