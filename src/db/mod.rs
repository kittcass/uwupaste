use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

mod model;
pub use model::*;

pub mod schema;

pub fn insert_paste(paste: &Paste, conn: &PgConnection) -> anyhow::Result<()> {
    diesel::insert_into(schema::paste::table)
        .values(paste)
        .execute(conn)?;
    Ok(())
}

pub fn retrieve_paste(search_id: &Uuid, conn: &PgConnection) -> anyhow::Result<Paste> {
    use schema::paste::dsl::*;
    Ok(paste.filter(id.eq(search_id)).first::<Paste>(conn)?)
}
