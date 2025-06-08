// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        is_active -> Nullable<Bool>,
        email_verified -> Nullable<Bool>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        stripe_customer_id -> Nullable<Varchar>,
        email_validation_token -> Nullable<Uuid>,
        email_validation_token_issued_at -> Nullable<Timestamptz>,
        password_reset_token -> Nullable<Uuid>,
        password_reset_token_issued_at -> Nullable<Timestamptz>,
    }
}
