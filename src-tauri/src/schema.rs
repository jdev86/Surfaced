// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        body -> Text,
        done -> Bool,
    }
}
