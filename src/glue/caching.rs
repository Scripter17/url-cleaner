use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = cache)]
pub struct CacheRow {
    id: u64,
    category: String,
    key: String,
    value: String
}

diesel::table! {
    cache (id) {
        id -> Integer,
        category -> Text,
        key -> Text,
        value -> Text
    }
}
