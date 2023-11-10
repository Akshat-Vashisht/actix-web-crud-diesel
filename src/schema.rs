// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 100]
        category -> Nullable<Varchar>,
        published -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
