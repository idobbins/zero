use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub stripe_customer_id: Option<String>,
    pub email_validation_token: Option<Uuid>,
    pub email_validation_token_issued_at: Option<DateTime<Utc>>,
    pub password_reset_token: Option<Uuid>,
    pub password_reset_token_issued_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub is_active: Option<bool>,
    pub email_verified: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub email_validation_token: Option<Uuid>,
    pub email_validation_token_issued_at: Option<DateTime<Utc>>,
}
