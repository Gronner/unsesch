-- SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
--
-- SPDX-License-Identifier: Apache-2.0 OR MIT

CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    capacity INTEGER NOT NULL,
    presentation BOOLEAN NOT NULL DEFAULT FALSE,
    drawing BOOLEAN NOT NULL DEFAULT FALSE,
    coding BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    start_time TIMESTAMP,
    end_time TIMESTAMP
);

CREATE TABLE schedules (
    id SERIAL PRIMARY KEY,
    reference TEXT NOT NULL UNIQUE
);

CREATE TABLE operators (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
