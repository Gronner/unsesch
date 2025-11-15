// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub capacity: i32,
    pub presentation: bool,
    pub drawing: bool,
    pub coding: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRoom<'a> {
    pub name: &'a str,
    pub capacity: i32,
    pub presentation: Option<bool>,
    pub drawing: Option<bool>,
    pub coding: Option<bool>,
}
