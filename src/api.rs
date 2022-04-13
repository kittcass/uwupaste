use std::{convert::Infallible, io::Cursor, ops::FromResidual};

use rocket::{
    form::Form,
    fs::TempFile,
    futures::TryFutureExt,
    http::{ContentType, Status},
    log::private::warn,
    response,
    response::Responder,
    serde::json::{serde_json::json, Json},
    Request, Response, Route,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    db::{insert_paste, retrieve_paste, NewPaste, Paste, PasteType},
    UwuPasteDbConn,
};

pub type ApiError<'a> = (Status, Option<&'a str>);

struct ApiResult<'a, T: Serialize>(std::result::Result<T, ApiError<'a>>);

impl<'a, T: Serialize> From<std::result::Result<T, ApiError<'a>>> for ApiResult<'a, T> {
    fn from(source: std::result::Result<T, ApiError<'a>>) -> Self {
        Self(source)
    }
}

impl<'a, T: Serialize> FromResidual<Result<Infallible, (Status, std::option::Option<&'a str>)>>
    for ApiResult<'a, T>
{
    fn from_residual(residual: Result<Infallible, (Status, std::option::Option<&'a str>)>) -> Self {
        Self(Err(residual.unwrap_err()))
    }
}

impl<'a, 'r, T: Serialize> Responder<'r, 'static> for ApiResult<'a, T> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = match &self.0 {
            Ok(value) => json!({
                "success": true,
                "value": value,
            }),
            Err((_, details)) => json!({
                "success": false,
                "details": details,
            }),
        };
        let body = rocket::serde::json::serde_json::to_string(&body).unwrap(); // TODO

        let status = match self.0 {
            Ok(_) => Status::Ok,
            Err((status, _)) => status,
        };

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

#[post("/paste", data = "<value>")]
async fn post_paste<'a>(value: String, conn: UwuPasteDbConn) -> ApiResult<'a, Paste> {
    let paste = NewPaste::new(PasteType::Text, value).build();
    let paste_clone = paste.clone();
    conn.run(move |conn| insert_paste(&paste, conn))
        .await
        .map_err(|e| {
            warn!("Unable to insert paste: {:?}", e);
            (Status::InternalServerError, None)
        })?;
    ApiResult(Ok(paste_clone))
}

#[derive(FromForm)]
struct PasteForm<'a> {
    file: TempFile<'a>,
}

#[post("/paste", data = "<value>")]
async fn upload_paste<'a>(
    value: Form<PasteForm<'_>>,
    conn: UwuPasteDbConn,
) -> ApiResult<'a, Paste> {
    unimplemented!()
}

#[get("/paste/<id>")]
async fn get_paste<'a>(id: &str, conn: UwuPasteDbConn) -> ApiResult<'a, Paste> {
    let id = Uuid::parse_str(id).map_err(|_| (Status::BadRequest, Some("Invalid UUID")))?;
    let paste = conn.run(move |conn| retrieve_paste(&id, conn)).await.ok();
    match paste {
        Some(paste) => ApiResult(Ok(paste)),
        None => ApiResult(Err((Status::NotFound, None))),
    }
}

#[delete("/<id>?<delete_code>")]
fn delete_paste(id: &str, delete_code: &str) {
    unimplemented!()
}

pub fn routes() -> Vec<Route> {
    routes![post_paste, get_paste]
}
