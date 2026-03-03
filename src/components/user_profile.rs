use std::rc::Rc;

use gpui::{AnyElement, App, Window, div, prelude::*};

use crate::app::LoginStatus;
use crate::auth::types::PlayerProfile;
use crate::theme::Colors;

#[derive(IntoElement)]
pub struct UserProfile {
    profile: Option<PlayerProfile>,
    login_status: LoginStatus,
    on_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    on_cancel_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    on_logout_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl UserProfile {
    pub fn new(
        profile: Option<PlayerProfile>,
        login_status: LoginStatus,
        on_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
        on_cancel_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
        on_logout_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            profile,
            login_status,
            on_login_click,
            on_cancel_login_click,
            on_logout_click,
        }
    }
}

impl RenderOnce for UserProfile {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self.profile {
            Some(profile) => Self::render_profile(profile, self.on_logout_click),
            None => Self::render_login_area(
                self.login_status,
                self.on_login_click,
                self.on_cancel_login_click,
            ),
        }
    }
}

impl UserProfile {
    fn render_profile(
        profile: PlayerProfile,
        on_logout_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    ) -> AnyElement {
        let initial = profile.name.chars().next().unwrap_or('?').to_string();

        div()
            .id("user-profile")
            .flex()
            .items_center()
            .gap_2()
            .px_2()
            .py_2()
            .child(
                div()
                    .size(gpui::px(32.))
                    .bg(Colors::secondary())
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .child(initial),
            )
            .child(
                div()
                    .flex_1()
                    .text_sm()
                    .child(profile.name),
            )
            .child(
                div()
                    .id("logout-button")
                    .px_1()
                    .text_sm()
                    .text_color(Colors::destructive())
                    .hover(|style| style.text_color(Colors::destructive_hover()))
                    .cursor_pointer()
                    .on_mouse_down(gpui::MouseButton::Left, move |_event, window, cx| {
                        (on_logout_click)(window, cx);
                    })
                    .child("Log Out"),
            )
            .into_any_element()
    }

    fn render_login_area(
        login_status: LoginStatus,
        on_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
        on_cancel_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    ) -> AnyElement {
        match login_status {
            LoginStatus::Idle => Self::render_button(
                "login-button",
                "Log In",
                Colors::muted(),
                Some(Colors::accent()),
                on_login_click,
            ),
            LoginStatus::WaitingForBrowser => {
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_3()
                            .py_2()
                            .bg(Colors::muted())
                            .text_sm()
                            .child("Logging in..."),
                    )
                    .child(Self::render_button(
                        "cancel-login-button",
                        "Cancel",
                        Colors::muted(),
                        Some(Colors::accent()),
                        on_cancel_click,
                    ))
                    .into_any_element()
            }
            LoginStatus::LoadingProfile => {
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .px_3()
                    .py_2()
                    .bg(Colors::muted())
                    .text_sm()
                    .child("Loading profile...")
                    .into_any_element()
            }
        }
    }

    fn render_button(
        id: impl Into<gpui::SharedString>,
        label: &str,
        bg: gpui::Rgba,
        hover_bg: Option<gpui::Rgba>,
        on_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    ) -> AnyElement {
        let mut el = div()
            .id(gpui::ElementId::Name(id.into()))
            .flex()
            .items_center()
            .justify_center()
            .px_3()
            .py_2()
            .bg(bg)
            .cursor_pointer()
            .text_sm()
            .on_mouse_down(gpui::MouseButton::Left, move |_event, window, cx| {
                (on_click)(window, cx);
            })
            .child(label.to_string());

        if let Some(hover_color) = hover_bg {
            el = el.hover(move |style| style.bg(hover_color));
        }

        el.into_any_element()
    }
}
