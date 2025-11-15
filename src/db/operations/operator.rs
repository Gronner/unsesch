// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use password_hash::PasswordHash;

use crate::db::{
    models::{NewOperator, Operator},
    schema::operators,
    trait_util::SqlPasswordHash,
};

pub async fn add_new_operator(
    conn: &mut AsyncPgConnection,
    username: &str,
    password: &PasswordHash<'_>,
) -> Operator {
    let new_operator = NewOperator {
        username,
        password: &SqlPasswordHash(password.serialize()),
    };
    diesel::insert_into(operators::table)
        .values(&new_operator)
        .returning(Operator::as_returning())
        .get_result(conn)
        .await
        .unwrap()
}

pub async fn get_operator(conn: &mut AsyncPgConnection, username: &str) -> Operator {
    operators::table
        .filter(operators::username.eq(username))
        .first(conn)
        .await
        .unwrap()
}

pub async fn remove_operator(conn: &mut AsyncPgConnection, operator_id: i32) {
    diesel::delete(operators::table)
        .filter(operators::id.eq(operator_id))
        .execute(conn)
        .await
        .unwrap();
}
