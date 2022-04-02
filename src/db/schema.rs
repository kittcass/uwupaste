table! {
    use diesel::sql_types::*;
    use crate::db::model::Paste_type;

    paste (id) {
        id -> Uuid,
        #[sql_name = "type"]
        type_ -> Paste_type,
        value -> Text,
        timestamp -> Nullable<Timestamptz>,
        expiration -> Nullable<Timestamptz>,
    }
}
