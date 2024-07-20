// @generated automatically by Diesel CLI.

diesel::table! {
    cache (id) {
        id -> Integer,
        category -> Text,
        key -> Text,
        value -> Nullable<Text>,
    }
}
