use gpui::{div, prelude::*};

use crate::theme::Colors;

pub struct CreatePage;

impl CreatePage {
    pub fn render() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap_2()
            .child(
                div()
                    .text_xl()
                    .text_color(Colors::foreground())
                    .child("Create Instance"),
            )
            .child(
                div()
                    .text_color(Colors::muted_foreground())
                    .text_sm()
                    .child("Instance creation form coming soon."),
            )
    }
}
