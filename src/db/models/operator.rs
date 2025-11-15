// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;

use crate::db::trait_util::SqlPasswordHash;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::operators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Operator {
    pub id: i32,
    pub username: String,
    pub password: SqlPasswordHash,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::operators)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewOperator<'a> {
    pub username: &'a str,
    pub password: &'a SqlPasswordHash,
}
