// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Text>,
        #[max_length = 500]
        profile_picture_url -> Nullable<Varchar>,
        email -> Text,
        password -> Nullable<Text>,
        oauth_provider -> Text,
        oauth_user_id -> Text,
        access_token -> Text,
        refresh_token -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Nullable<Timestamptz>,
    }
}
