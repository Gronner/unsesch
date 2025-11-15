// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::session_room)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionRoomMapping {
    pub room_id: i32,
    pub session_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::session_room)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSessionRoomMapping {
    pub room_id: i32,
    pub session_id: i32,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::session_schedule)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionScheduleMapping {
    pub schedule_id: i32,
    pub session_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::session_schedule)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSessionScheduleMapping {
    pub schedule_id: i32,
    pub session_id: i32,
}
