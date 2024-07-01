// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        telegram_id -> Int8,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        username -> Nullable<Text>,
        language_code -> Nullable<Text>,
        is_premium -> Nullable<Bool>,
        photo_url -> Nullable<Text>,
    }
}
