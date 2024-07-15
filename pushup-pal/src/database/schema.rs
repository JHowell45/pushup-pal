// @generated automatically by Diesel CLI.

diesel::table! {
    pushup_sessions (id) {
        id -> Text,
        amount -> Integer,
        created_at -> Timestamp,
    }
}
