use chrono::{DateTime, Duration, Utc};
use diesel::Queryable;
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use super::schema::paste;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, DbEnum, AsExpression)]
#[serde(rename_all = "snake_case")]
#[DieselType = "Paste_type"]
pub enum PasteType {
    Text,
    File,
}

#[derive(Debug, Clone, Serialize, Insertable, Queryable, Identifiable)]
#[table_name = "paste"]
pub struct Paste {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_: PasteType,
    pub value: String,
    pub timestamp: Option<DateTime<Utc>>,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewPaste {
    type_: PasteType,
    value: String,
    expiration_duration: Option<Duration>,
}

impl NewPaste {
    pub fn new(type_: PasteType, value: impl Into<String>) -> Self {
        Self {
            type_,
            value: value.into(),
            expiration_duration: None,
        }
    }

    pub fn expiration_duration(mut self, expiration_duration: Duration) -> Self {
        self.expiration_duration = Some(expiration_duration);
        self
    }

    pub fn build(self) -> Paste {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let expiration = self.expiration_duration.map(|d| now + d);

        Paste {
            id,
            type_: self.type_,
            value: self.value,
            timestamp: Some(now),
            expiration,
        }
    }
}
