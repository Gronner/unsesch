// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use icondata;
use leptos::{component, prelude::*, IntoView};
use thaw::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Button
            size=ButtonSize::Large
            appearance=ButtonAppearance::Transparent
            on_click=move |_| open.update(|open| *open = !*open)
        >
            <Icon icon=icondata::AiMenuOutlined />
        </Button>
        <OverlayDrawer open modal_type=DrawerModalType::NonModal>
            <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            appearance=ButtonAppearance::Transparent
                            icon=icondata::ImCross
                            on_click=move |_| open.set(false)
                        />
                    </DrawerHeaderTitleAction>
                    "Unsesch"
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <Flex>
                    <NavDrawer>
                        <NavItem icon=icondata::BiHomeAlt2Regular value="home" href="/">
                            Home
                        </NavItem>
                        <NavItem
                            icon=icondata::FaCalendarDaysRegular
                            value="schedule"
                            href="/schedule"
                        >
                            Schedule
                        </NavItem>
                        <NavItem
                            icon=icondata::BiDirectionsRegular
                            value="room_plan"
                            href="/room_plan"
                        >
                            Room Plan
                        </NavItem>
                        <NavItem
                            icon=icondata::AiGithubOutlined
                            value="github"
                            href="https://github.com/Gronner/unsesch"
                            attr:target="_blank"
                        >
                            Github
                        </NavItem>
                    </NavDrawer>
                </Flex>
            </DrawerBody>
        </OverlayDrawer>
    }
}
