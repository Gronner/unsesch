// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::{
    models::{NewSessionScheduleMapping, Schedule, Session, SessionScheduleMapping},
    schema::session_schedule,
};

pub async fn assign_session_to_schedule(
    conn: &mut AsyncPgConnection,
    session: &Session,
    schedule: &Schedule,
) -> SessionScheduleMapping {
    let new_mapping = NewSessionScheduleMapping {
        schedule_id: schedule.id,
        session_id: session.id,
    };
    diesel::insert_into(session_schedule::table)
        .values(&new_mapping)
        .returning(SessionScheduleMapping::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}
