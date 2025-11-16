// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SessionCard(
    name: &'static str,
    time: &'static str,
    presenter: &'static str,
    description: &'static str,
) -> impl IntoView {
    let icon = RwSignal::new(Some(icondata::AiHeartOutlined));

    let on_click = move |_| {
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

    view! {
        <Card>
            <CardHeader>
                <Body1>
                    <b>{name}</b>
                </Body1>
                <CardHeaderDescription slot>
                    <Flex align=FlexAlign::Start gap=FlexGap::Medium>
                        <Caption1>{presenter}</Caption1>
                        <Caption1>{time}</Caption1>
                    </Flex>
                </CardHeaderDescription>
                <CardHeaderAction slot>
                    <Button appearance=ButtonAppearance::Transparent icon on_click />
                </CardHeaderAction>
            </CardHeader>
            <CardFooter>
                <DescriptionDialog
                    name=name
                    time=time
                    presenter=presenter
                    description=description
                />
            </CardFooter>
        </Card>
    }
}

#[component]
pub fn DescriptionDialog(
    name: &'static str,
    time: &'static str,
    presenter: &'static str,
    description: &'static str,
) -> impl IntoView {
    let open = RwSignal::new(false);

    view! {
        <Button on_click=move |_| open.set(true)>"Description"</Button>
        <Dialog open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{name}</DialogTitle>
                    <DialogContent>
                        <h2>{presenter}</h2>
                        <h3>{time}</h3>
                        <p>{description}</p>
                    </DialogContent>
                    <DialogActions>
                        <Button
                            appearance=ButtonAppearance::Primary
                            on_click=move |_| open.set(false)
                        >
                            "Close"
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
