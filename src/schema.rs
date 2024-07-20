// @generated automatically by Diesel CLI.

diesel::table! {
    cache (id) {
        id -> Integer,
        category -> Text,
        k -> Text,
        value -> Text,
    }
}
