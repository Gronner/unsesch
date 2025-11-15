// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSession<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
}
