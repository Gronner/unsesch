// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid_readable_rs::short;

use crate::db::{
    models::{NewSchedule, Schedule},
    schema::schedules,
};

pub async fn add_new_schedule(conn: &mut AsyncPgConnection) -> Schedule {
    let new_schedule = NewSchedule {
        reference: &short().replace(' ', "-"),
    };
    diesel::insert_into(schedules::table)
        .values(&new_schedule)
        .returning(Schedule::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_schedules(conn: &mut AsyncPgConnection) -> Vec<Schedule> {
    schedules::table.load(conn).await.unwrap()
}

pub async fn get_schedule_by_id(conn: &mut AsyncPgConnection, schedule_id: i32) -> Schedule {
    schedules::table
        .find(schedule_id)
        .first(conn)
        .await
        .unwrap()
}

pub async fn get_schedule(conn: &mut AsyncPgConnection, reference: &str) -> Schedule {
    schedules::table
        .filter(schedules::reference.eq(reference))
        .first(conn)
        .await
        .unwrap()
}

pub async fn remove_schedule(conn: &mut AsyncPgConnection, schedule_id: i32) {
    diesel::delete(schedules::table)
        .filter(schedules::id.eq(schedule_id))
        .execute(conn)
        .await
        .unwrap();
}
