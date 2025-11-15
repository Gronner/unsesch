// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// @generated automatically by Diesel CLI.

diesel::table! {
    operators (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        name -> Text,
        capacity -> Int4,
        presentation -> Bool,
        drawing -> Bool,
        coding -> Bool,
    }
}

diesel::table! {
    schedules (id) {
        id -> Int4,
        reference -> Text,
    }
}

diesel::table! {
    session_room (room_id, session_id) {
        room_id -> Int4,
        session_id -> Int4,
    }
}

diesel::table! {
    session_schedule (schedule_id, session_id) {
        schedule_id -> Int4,
        session_id -> Int4,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
    }
}

diesel::joinable!(session_room -> rooms (room_id));
diesel::joinable!(session_room -> sessions (session_id));
diesel::joinable!(session_schedule -> schedules (schedule_id));
diesel::joinable!(session_schedule -> sessions (session_id));

diesel::allow_tables_to_appear_in_same_query!(
    operators,
    rooms,
    schedules,
    session_room,
    session_schedule,
    sessions,
);
