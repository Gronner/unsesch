// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::Write;

use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    pg::Pg,
    serialize::{IsNull, ToSql},
    sql_types::Text,
};
use password_hash::PasswordHashString;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = Text)]
pub struct SqlPasswordHash(pub PasswordHashString);

impl ToSql<Text, Pg> for SqlPasswordHash
where
    String: ToSql<Text, Pg>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for SqlPasswordHash
where
    String: ToSql<Text, Pg>,
{
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(SqlPasswordHash(
            PasswordHashString::new(&<String as FromSql<Text, Pg>>::from_sql(bytes)?).unwrap(),
        ))
    }
}
