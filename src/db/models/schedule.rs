// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Schedule {
    pub id: i32,
    pub reference: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSchedule<'a> {
    pub reference: &'a str,
}
