use gpui::{AnyElement, App, Window, div, prelude::*};

use crate::theme::Colors;

pub struct User {
    pub username: String,
}

#[derive(IntoElement)]
pub struct UserProfile {
    user: Option<User>,
}

impl UserProfile {
    pub fn new(user: Option<User>) -> Self {
        Self { user }
    }
}

impl RenderOnce for UserProfile {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        match self.user {
            Some(user) => Self::render_profile(user),
            None => Self::render_login_button(),
        }
    }
}

impl UserProfile {
    fn render_profile(user: User) -> AnyElement {
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
                    .child(user.username.chars().next().unwrap_or('?').to_string()),
            )
            .child(user.username)
            .into_any_element()
    }

    fn render_login_button() -> AnyElement {
        div()
            .id("login-button")
            .flex()
            .items_center()
            .justify_center()
            .px_3()
            .py_2()
            .bg(Colors::muted())
            .hover(|style| style.bg(Colors::accent()))
            .cursor_pointer()
            .text_sm()
            .child("Log In")
            .into_any_element()
    }
}
