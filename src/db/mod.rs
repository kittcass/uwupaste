use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::database;
use uuid::Uuid;

mod model;
pub use model::*;

pub mod schema;

#[database("uwupaste")]
pub struct UwuPasteDbConn(diesel::PgConnection);

pub fn insert_paste(paste: &Paste, conn: &PgConnection) -> anyhow::Result<()> {
    use schema::paste;
    diesel::insert_into(paste::table)
        .values(paste)
        .execute(conn)?;
    Ok(())
}

pub fn retrieve_paste(search_id: &Uuid, conn: &PgConnection) -> anyhow::Result<Paste> {
    use schema::paste::dsl::*;
    Ok(paste.filter(id.eq(search_id)).first::<Paste>(conn)?)
}
