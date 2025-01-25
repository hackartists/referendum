#![allow(unused)]
use crate::QueryResponse;
use crate::*;
use by_macros::api_model;

#[cfg(feature = "server")]
use by_axum::aide;
use lazy_static::lazy_static;
use validator::ValidationError;

#[derive(validator::Validate)]
#[api_model(base = "/users/v1", read_action = user_info, table = users, iter_type=QueryResponse)]
pub struct User {
    #[api_model(primary_key)]
    pub id: String,
    #[api_model(auto = insert)]
    pub created_at: u64,
    #[api_model(auto = [insert, update])]
    pub updated_at: u64,

    #[api_model(action = signup)]
    #[validate(custom(function = "validate_nickname"))]
    pub nickname: String,
    #[api_model(action = signup, read_action = [check_email, login], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup)]
    #[validate(url)]
    pub profile_url: String,

    #[api_model(type = INTEGER)]
    pub role: UserRole,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin = 0,
    #[default]
    User = 1,
}

impl TryFrom<i32> for UserRole {
    type Error = String;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(UserRole::Admin),
            1 => Ok(UserRole::User),
            _ => Err(format!("Invalid UserRole: {}", value)),
        }
    }
}

impl Into<i32> for UserRole {
    fn into(self) -> i32 {
        self as i32
    }
}

#[cfg(feature = "server")]
impl sqlx::Type<sqlx::Postgres> for UserRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "server")]
impl sqlx::Encode<'_, sqlx::Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let value: i32 = (*self).clone().into();
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
    }
}

#[cfg(feature = "server")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserRole {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let int_value: i32 = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        UserRole::try_from(int_value)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)).into())
    }
}

fn validate_nickname(nickname: &str) -> std::result::Result<(), ValidationError> {
    lazy_static! {
        static ref NICKNAME_REGEX: regex::Regex =
            regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_]{1,20}$").unwrap();
    }

    if !NICKNAME_REGEX.is_match(nickname) {
        return Err(ValidationError::new("Nickname must be started with alphabet or number and only allow alphabet, number, hyphen and underscore, maximum 20 characters"));
    }

    Ok(())
}
