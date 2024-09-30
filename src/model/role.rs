use std::{io::Write, str::FromStr};

use chrono::NaiveDateTime;
use diesel::{deserialize::{FromSql, FromSqlRow}, expression::AsExpression, pg::{Pg, PgValue}, prelude::*, serialize::{Output, ToSql}, sql_types::Text};
use serde::{Deserialize, Serialize};
use crate::schema::roles;

#[derive(AsExpression, Debug, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Trainer,
    Student,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name= roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(AsChangeset, Insertable, Deserialize)]
#[diesel(table_name = roles)]
pub struct UpdateRole {
    pub code: Option<RoleCode>,
    pub name: Option<String>
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "trainer" => Ok(RoleCode::Trainer),
            "student" => Ok(RoleCode::Student),
            _ => Ok(RoleCode::Student),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCode::Admin),
            b"trainer" => Ok(RoleCode::Trainer),
            b"student" => Ok(RoleCode::Student),
            _ => Ok(RoleCode::Student),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Trainer => out.write_all(b"trainer")?,
            RoleCode::Student => out.write_all(b"student")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}
