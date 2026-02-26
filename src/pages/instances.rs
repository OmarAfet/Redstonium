use gpui::{div, prelude::*, px};

use crate::theme::Colors;

pub struct InstancesPage;

impl InstancesPage {
    pub fn render() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
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
                    .child("Your installed Minecraft instances."),
            )
            .children((0..30).map(|i| {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py_3()
                    .bg(Colors::sidebar())
                    .border_1()
                    .border_color(Colors::border())
                    .child(
                        div()
                            .size(px(40.))
                            .bg(Colors::secondary())
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .child(format!("{}", i + 1)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(Colors::foreground())
                                    .child(format!("Minecraft 1.{}.{}", 20 - (i % 10), i % 5)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(Colors::muted_foreground())
                                    .child(if i % 3 == 0 { "Fabric" } else if i % 3 == 1 { "Forge" } else { "Vanilla" }),
                            ),
                    )
            }))
    }
}
