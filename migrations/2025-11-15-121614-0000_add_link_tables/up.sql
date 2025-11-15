-- SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
--
-- SPDX-License-Identifier: Apache-2.0 OR MIT

-- Your SQL goes here

CREATE TABLE session_room (
    room_id INTEGER REFERENCES rooms,
    session_id INTEGER REFERENCES sessions,
    PRIMARY KEY (room_id, session_id)
);

CREATE TABLE session_schedule (
    schedule_id INTEGER REFERENCES schedules,
    session_id INTEGER REFERENCES sessions,
    PRIMARY KEY (schedule_id, session_id)
);
