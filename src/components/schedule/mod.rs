// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use leptos::{prelude::*, reactive::spawn_local};
use leptos_meta::Style;
use serde::{Deserialize, Serialize};
use thaw::*;

mod room_schedule;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Session {
    id: i32,
    title: String,
    description: String,
    presenter: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Room {
    id: i32,
    name: String,
    capacity: i32,
    presentation: bool,
    drawing: bool,
    coding: bool,
    sessions: Vec<i32>,
}

#[server]
async fn get_session_schedule() -> Result<Vec<Room>, ServerFnError> {
    let mut schedule = Vec::new();
    schedule.push(Room {
        id: 0,
        name: String::from("Room A"),
        capacity: 20,
        presentation: false,
        drawing: false,
        coding: false,
        sessions: vec![0, 1],
    });
    schedule.push(Room {
        id: 1,
        name: String::from("Room B"),
        capacity: 15,
        presentation: true,
        drawing: true,
        coding: true,
        sessions: vec![2],
    });
    Ok(schedule)
}

use room_schedule::RoomSchedule;

#[component]
pub fn RoomSchedules() -> impl IntoView {
    let (rooms, rooms_set) = signal(Vec::<Room>::new());

    Effect::new(move |_| {
        spawn_local(async move {
            rooms_set.set(get_session_schedule().await.unwrap());
        })
    });

    let offset = Local::now().offset().clone();

    let from = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2025, 11, 17).unwrap(),
        NaiveTime::from_hms_opt(7, 0, 0).unwrap(),
    );
    let start_time = DateTime::<Local>::from_naive_utc_and_offset(from, offset);
    let from = from + Duration::minutes(45);
    let end_time = DateTime::<Local>::from_naive_utc_and_offset(from, offset);
    view! {
        <Style>
            "@media screen and (min-width:700px) {
              .schedule {
                display: grid;
                grid-gap: 1em;
                grid-template-rows:
                  [tracks] auto
                  [time-0800] 1fr
                  [time-0815] 1fr
                  [time-0830] 1fr
                  [time-0845] 1fr
                  [time-0900] 1fr
                  [time-0915] 1fr
                  [time-0930] 1fr
                  [time-0945] 1fr
                  [time-1000] 1fr
                  [time-1015] 1fr
                  [time-1030] 1fr
                  [time-1045] 1fr
                  [time-1100] 1fr
                  [time-1115] 1fr
                  [time-1130] 1fr
                  [time-1145] 1fr
                  [time-1200] 1fr;
            
                grid-template-columns:
                  [times] 4em
                  [track-1-start] 1fr
                  [track-1-end track-2-start] 1fr
                  [track-2-end track-3-start] 1fr
                  [track-3-end track-4-start] 1fr
                  [track-4-end];
              }
            }
            
            .time-slot {
              grid-column: times;
            }
            
            .track-slot {
              display: none;
            }
            
            @supports( display:grid ) {
              @media screen and (min-width:700px) {
                .track-slot {
                  display: block;
                  padding: 10px 5px 5px;
                  position: sticky;
                  top: 0;
                  z-index: 1000;
                  background-color: rgba(255,255,255,.9);
                }
              }
            }
            
            .session {
              z-index: 1;
              margin-bottom:  1em;
            }
            
            .trackbg {
                z-index: 0;
                opacity: 0.3;
            }
            
            @supports( display:grid ) {
              @media screen and (min-width: 700px) {
                .session {
                  margin: 0;
                }
              }
            }
            
            body {
              padding: 50px;
              max-width: 1100px;
              margin: 0 auto;
              line-height: 1.5;
            }
            
            .session {
              padding: .5em;
              border-radius: 2px;
              font-size: 14px;
            }
            
            .session-title,
            .session-time,
            .session-track,
            .session-presenter {
              display: block;
            }
            
            .session-title,
            .time-slot {
              margin: 0;
              font-size: 1em;
            }
            
            .session-title a {
              color: #fff;
              text-decoration-style: dotted;
            
              &:hover {
                font-style: italic;
              }
            
              &:focus {
                outline: 2px dotted rgba(255,255,255,.8);
              }
            }
            
            .track-slot,
            .time-slot {
              font-weight: bold;
              font-size:.75em;
            }
            
            .track-1 {
              background-color: #1259B2;
              color: #fff;
            }
            
            .track-2 {
              background-color: #687f00;
              color: #fff;
            }
            
            .track-3 {
              background-color: #544D69;
              color: #fff;
            }
            
            .track-4 {
              background-color: #c35500;
              color: #fff;
            }
            
            .track-all {
              display: flex;
              justify-content: center;
              align-items: center;
              background: #ccc;
              color: #000;
              box-shadow: none;
            }
            
            .text {
              max-width: 750px;
              font-size: 18px;
              margin: 0 auto 50px;
            }
            
            .meta {
              color: #555;
              font-style: italic;
            }
            
            .meta a {
              color: #555;
            }
            
            hr {
              margin: 40px 0;
            }"
        </Style>
        <div class="schedule" aria-labelledby="schedule-heading">
            <div
                class="trackbg track-1"
                style="grid-column: track-1; grid-row: time-0800 / time-1200;"
            />
            <div
                class="trackbg track-2"
                style="grid-column: track-2; grid-row: time-0800 / time-1200;"
            />
            <div
                class="trackbg track-3"
                style="grid-column: track-3; grid-row: time-0800 / time-1200;"
            />
            <div
                class="trackbg track-4"
                style="grid-column: track-4; grid-row: time-0800 / time-1200;"
            />

            <span
                class="track-slot track-1"
                aria-hidden="true"
                style="grid-column: track-1; grid-row: tracks;"
            >
                Track 1
            </span>
            <span
                class="track-slot track-2"
                aria-hidden="true"
                style="grid-column: track-2; grid-row: tracks;"
            >
                Track 2
            </span>
            <span
                class="track-slot track-3"
                aria-hidden="true"
                style="grid-column: track-3; grid-row: tracks;"
            >
                Track 3
            </span>
            <span
                class="track-slot track-4"
                aria-hidden="true"
                style="grid-column: track-4; grid-row: tracks;"
            >
                Track 4
            </span>

            <h2 class="time-slot" style="grid-row: time-0800;">
                8:00
            </h2>

            <Session
                id=1
                room=String::from("Track 1")
                title=String::from("Session Title")
                description=String::from("Short Text")
                presenter=String::from("Horst")
                start_time=start_time
                end_time=end_time
            />

            <div
                class="session session-3 track-3"
                style="grid-column: track-3; grid-row: time-0800 / time-0830;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">8:00 - 8:30</span>
                <span class="session-track">Track: 3</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <div
                class="session session-4 track-4"
                style="grid-column: track-4; grid-row: time-0800 / time-1000;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">8:00 - 10:00</span>
                <span class="session-track">Track: 2</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <div
                class="session session-5 track-3"
                style="grid-column: track-3; grid-row: time-0830 / time-1000;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">8:30 - 10:00</span>
                <span class="session-track">Track: 1</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <h2 class="time-slot" style="grid-row: time-0900;">
                9:00
            </h2>

            <div
                class="session session-6 track-1"
                style="grid-column: track-1-start / track-2-end; grid-row: time-0900 / time-1000;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">9:00 - 10:00</span>
                <span class="session-track">Track: 1 & 2</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <h2 class="time-slot" style="grid-row: time-1000;">
                10:00
            </h2>

            <div
                class="session session-7 track-all"
                style="grid-column: track-1-start / track-4-end; grid-row: time-1000 / time-1030;"
            >
                <h3 class="session-title">Take a break!</h3>
            </div>

            <div
                class="session session-8 track-1"
                style="grid-column: track-1; grid-row: time-1030 / time-1130;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">10:30 - 11:30</span>
                <span class="session-track">Track: 1</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <div
                class="session session-9 track-2"
                style="grid-column: track-2-start / track-3-end; grid-row: time-1030 / time-1100;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">10:30 - 11:00</span>
                <span class="session-track">Track: 2 & 3</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <div
                class="session session-10 track-4"
                style="grid-column: track-4; grid-row: time-1030 / time-1100;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">10:30 - 11:00</span>
                <span class="session-track">Track: 4</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <h2 class="time-slot" style="grid-row: time-1100;">
                11:00
            </h2>

            <div
                class="session session-11 track-2"
                style="grid-column: track-2; grid-row: time-1100 / time-1200;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">11:00 - 12:00</span>
                <span class="session-track">Track: 2</span>
                <span class="session-presenter">Presenter</span>
            </div>

            <div
                class="session session-11 track-3"
                style="grid-column: track-3; grid-row: time-1100 / time-1200;"
            >
                <h3 class="session-title">
                    <a href="#">Talk Title</a>
                </h3>
                <span class="session-time">11:00 - 12:00</span>
                <span class="session-track">Track: 3</span>
                <span class="session-presenter">Presenter</span>
            </div>
        </div>
    }
}

#[component]
fn Session(
    id: i32,
    room: String,
    title: String,
    description: String,
    presenter: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
) -> impl IntoView {
    let open = RwSignal::new(false);

    let icon = RwSignal::new(Some(icondata::AiHeartOutlined));
    let on_like = move |_| {
        icon.update(|icon| {
            *icon = match icon {
                Some(data) => {
                    if *data == icondata::AiHeartOutlined {
                        icondata::AiHeartFilled
                    } else {
                        icondata::AiHeartOutlined
                    }
                }
                None => icondata::AiHeartOutlined,
            }
            .into();
        });
    };

    let track = room.to_lowercase().replace(' ', "-");
    let time_slots = format!(
        "time-{} / time-{}",
        start_time.format("%H%M"),
        end_time.format("%H%M")
    );

    let start_time = format!("{}", start_time.time().format("%H:%M"));
    let end_time = format!("{}", end_time.time().format("%H:%M"));

    let class = format!("session session-{id} {track}");
    let style = format!("grid-column: {track}; grid-row: {time_slots};");

    view! {
        <div class=class style=style>
            <h3 class="session-title">{title.clone()}</h3>
            <span class="session-time">{start_time.clone()}- {end_time.clone()}</span>
            <span class="session-track">{room}</span>
            <span class="session-presenter">{presenter.clone()}</span>
            <Button on_click=move |_| open.set(true)>"Description"</Button>
            <Button appearance=ButtonAppearance::Transparent icon on_click=on_like />
            <Dialog open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{title.clone()}</DialogTitle>
                        <DialogContent>
                            <h2>{presenter.clone()}</h2>
                            <h3>{start_time}-{end_time}</h3>
                            <p>{description}</p>
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance=ButtonAppearance::Primary
                                on_click=move |_| open.set(false)
                            >
                                "Close"
                            </Button>
                            <Button appearance=ButtonAppearance::Primary icon on_click=on_like />
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </div>
    }
}
