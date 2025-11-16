// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::{prelude::*, IntoView};
use thaw::*;

use crate::components::{PageHeader, RoomSchedules, SessionCard};

#[component]
pub fn Schedule() -> impl IntoView {
    view! {
        <Layout position=LayoutPosition::Absolute>
            <PageHeader />
            <Layout attr:style="padding: 20px;">
                <SessionCard
                    name="Test"
                    time="10:00-10:45"
                    presenter="Horst"
                    description="A very lengthy description that hopefully does not fit the box I have chosen for this. But lets see how fast I will run out of ideas of what to write and what not to write."
                />
                <RoomSchedules />
            </Layout>
        </Layout>
    }
}
