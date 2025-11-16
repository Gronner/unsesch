// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use thaw::*;

use crate::components::schedule::Room;

#[component]
pub fn RoomSchedule(room: Room) -> impl IntoView {
    view! {
        <Space vertical=true>
            <RoomHeader
                name=room.name
                capacity=room.capacity
                presentation=room.presentation
                drawing=room.drawing
                coding=room.coding
            />
            <RoomSessions sessions=room.sessions />
        </Space>
    }
}

#[component]
pub fn RoomHeader(
    name: String,
    capacity: i32,
    presentation: bool,
    drawing: bool,
    coding: bool,
) -> impl IntoView {
    view! {
        <Grid cols=1>
            <Text>{name}</Text>
            <div>
                <Grid cols=2>
                    <div>
                        <Icon icon=icondata::BsPeopleFill />
                        <Text>{capacity}</Text>
                    </div>
                    <div>
                        <Show when=move || presentation>
                            <Icon icon=icondata::MdiProjectorScreenOutline />
                        </Show>
                        <Show when=move || drawing>
                            <Icon icon=icondata::MdiDrawPen />
                        </Show>
                        <Show when=move || coding>
                            <Icon icon=icondata::MdiDesk />
                        </Show>
                    </div>
                </Grid>
            </div>
        </Grid>
    }
}

#[component]
pub fn RoomSessions(sessions: Vec<i32>) -> impl IntoView {
    view! {
        <Grid cols=1>
            <For each=move || sessions.clone() key=|&session| session let(session)>
                <Text>Session {session}</Text>
            </For>
        </Grid>
    }
}
