// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::{
    models::{NewRoom, Room},
    schema::rooms,
};

pub async fn add_room(
    conn: &mut AsyncPgConnection,
    name: &str,
    capacity: i32,
    presentation: Option<bool>,
    drawing: Option<bool>,
    coding: Option<bool>,
) -> Room {
    let new_room = NewRoom {
        name,
        capacity,
        presentation,
        drawing,
        coding,
    };
    diesel::insert_into(rooms::table)
        .values(&new_room)
        .returning(Room::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_rooms(conn: &mut AsyncPgConnection) -> Vec<Room> {
    rooms::table.load(conn).await.unwrap()
}

pub async fn get_room_by_id(conn: &mut AsyncPgConnection, room_id: i32) -> Room {
    rooms::table.find(room_id).first(conn).await.unwrap()
}

pub async fn get_room(conn: &mut AsyncPgConnection, name: &str) -> Room {
    rooms::table
        .filter(rooms::name.eq(name))
        .first(conn)
        .await
        .unwrap()
}

pub async fn remove_room(conn: &mut AsyncPgConnection, room_id: i32) {
    diesel::delete(rooms::table)
        .filter(rooms::id.eq(room_id))
        .execute(conn)
        .await
        .unwrap();
}
