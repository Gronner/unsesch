// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use thaw::*;

use crate::components::NavBar;

#[component]
pub fn PageHeader() -> impl IntoView {
    let theme = Theme::use_rw_theme(); //RwSignal::new(Theme::light());
    let icon = RwSignal::new(Some(icondata::BiSunRegular));

    let switch_theme = move |_| {
        icon.update(|icon| {
            *icon = match icon {
                Some(data) => {
                    if *data == icondata::BiSunRegular {
                        theme.set(Theme::dark());
                        icondata::BiMoonRegular
                    } else {
                        theme.set(Theme::light());
                        icondata::BiSunRegular
                    }
                }
                None => icondata::BiSunRegular,
            }
            .into();
        });
    };

    view! {
        <LayoutHeader attr:style="padding: 20px;">
            <Grid cols=3>
                <GridItem>
                    <NavBar />
                </GridItem>
                <GridItem>
                    <h1>"Unsesch"</h1>
                    <Image
                        src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg"
                        width="200px"
                        height="200px"
                        shape=ImageShape::Circular
                    />
                </GridItem>
                <GridItem>
                    <Button
                        size=ButtonSize::Large
                        appearance=ButtonAppearance::Transparent
                        icon
                        on_click=switch_theme
                    />
                </GridItem>
            </Grid>
        </LayoutHeader>
    }
}
