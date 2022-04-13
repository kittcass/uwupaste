use std::collections::HashMap;

use rocket::{Route, response::status, serde::json::Json, http::Status};
use rocket_dyn_templates::Template;
use uuid::Uuid;

use crate::db::{UwuPasteDbConn, retrieve_paste, Paste, PasteType};

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", &context)
}

#[get("/<id>")]
async fn get_paste(
    id: &str,
    conn: UwuPasteDbConn,
) -> status::Custom<Template> {
    // let id = Uuid::parse_str(id).map_err(|e| BadRequest(Some(e.to_string())))?;
    // let paste: Paste = conn.run(move |conn| retrieve_paste(&id, conn)).await.ok()?;
    // match paste.type_ {
    //     PasteType::Text => {

    //     }
    //     PasteType::File => unimplemented!(),
    // }
    // Ok(paste.map(Json))
    let context = HashMap::<String, String>::new();
    status::Custom(Status::NotFound, Template::render("paste/404", &context))
}

pub fn routes() -> Vec<Route> {
    routes![index, get_paste]
}
