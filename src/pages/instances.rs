use gpui::{div, prelude::*};

use crate::theme::Colors;

pub struct InstancesPage;

impl InstancesPage {
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
                    .child("Instances"),
            )
            .child(
                div()
                    .text_color(Colors::muted_foreground())
                    .text_sm()
                    .child("No instances installed yet."),
            )
    }
}
