// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use thaw::*;

use crate::components::PageHeader;

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <Layout position=LayoutPosition::Absolute>
            <PageHeader />
            <Layout attr:style="padding: 20px;">Test</Layout>
        </Layout>
    }
}
