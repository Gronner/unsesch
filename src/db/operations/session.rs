// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::{
    models::{NewSession, NewSessionRoomMapping, Room, Session, SessionRoomMapping},
    schema::{session_room, sessions},
};

pub async fn add_session(
    conn: &mut AsyncPgConnection,
    title: &str,
    description: Option<&str>,
    start_time: Option<NaiveDateTime>,
    end_time: Option<NaiveDateTime>,
) -> Session {
    let new_session = NewSession {
        title,
        description,
        start_time,
        end_time,
    };
    diesel::insert_into(sessions::table)
        .values(&new_session)
        .returning(Session::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn remove_session(conn: &mut AsyncPgConnection, session_id: i32) {
    diesel::delete(sessions::table)
        .filter(sessions::id.eq(session_id))
        .execute(conn)
        .await
        .unwrap();
}

pub async fn get_sessions(conn: &mut AsyncPgConnection) -> Vec<Session> {
    sessions::table.load(conn).await.unwrap()
}

pub async fn get_session_by_id(conn: &mut AsyncPgConnection, room_id: i32) -> Session {
    sessions::table.find(room_id).first(conn).await.unwrap()
}

pub async fn get_sessions_by_title(conn: &mut AsyncPgConnection, title: &str) -> Vec<Session> {
    sessions::table
        .filter(sessions::title.eq(title))
        .load(conn)
        .await
        .unwrap()
}

pub async fn assign_session_to_room(
    conn: &mut AsyncPgConnection,
    session: &Session,
    room: &Room,
) -> SessionRoomMapping {
    let new_mapping = NewSessionRoomMapping {
        room_id: room.id,
        session_id: session.id,
    };
    diesel::insert_into(session_room::table)
        .values(&new_mapping)
        .returning(SessionRoomMapping::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}
