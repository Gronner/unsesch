// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::{prelude::*, IntoView};
use thaw::*;

use crate::components::PageHeader;

#[component]
pub fn RoomPlan() -> impl IntoView {
    view! {
        <Layout position=LayoutPosition::Absolute>
            <PageHeader />
            <Layout attr:style="padding: 20px;">
                <p>This will be the room plan</p>
            </Layout>
        </Layout>
    }
}
