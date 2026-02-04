// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use thaw::{ssr::SSRMountStyleProvider, *};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

use crate::pages::Homepage;
use crate::pages::RoomPlan;
use crate::pages::Schedule;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/unsesch.css" />

        // sets the document title
        <Title text="Unsesch: The Unconference Session Scheduler" />

        <ConfigProvider>
            <Router>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=Homepage />
                    <Route path=StaticSegment("schedule") view=Schedule />
                    <Route path=StaticSegment("room_plan") view=RoomPlan />
                </Routes>
            </Router>
        </ConfigProvider>
    }
}
