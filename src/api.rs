use rocket::{
    form::Form, fs::TempFile, response::status::BadRequest, serde::json::Json, Data, Route,
};
use uuid::Uuid;

use crate::{
    db::{insert_paste, retrieve_paste, NewPaste, Paste, PasteType},
    UwuPasteDbConn,
};

#[post("/paste", data = "<value>")]
async fn post_paste(value: String, conn: UwuPasteDbConn) -> Json<Paste> {
    let paste = NewPaste::new(PasteType::Text, value).build();
    let paste_clone = paste.clone();
    conn.run(move |conn| insert_paste(&paste, conn))
        .await
        .expect("Could not insert paste");
    Json(paste_clone)
}

#[derive(FromForm)]
struct PasteForm<'a> {
    file: TempFile<'a>,
}

#[post("/paste", data = "<value>")]
async fn upload_paste(value: Form<PasteForm<'_>>, conn: UwuPasteDbConn) -> Json<Paste> {
    unimplemented!()
}

#[get("/paste/<id>")]
async fn get_paste(
    id: &str,
    conn: UwuPasteDbConn,
) -> Result<Option<Json<Paste>>, BadRequest<String>> {
    let id = Uuid::parse_str(id).map_err(|e| BadRequest(Some(e.to_string())))?;
    let paste = conn.run(move |conn| retrieve_paste(&id, conn)).await.ok();
    Ok(paste.map(Json))
}

#[delete("/<id>?<delete_code>")]
fn delete_paste(id: &str, delete_code: &str) {
    unimplemented!()
}

pub fn routes() -> Vec<Route> {
    routes![post_paste, get_paste]
}
