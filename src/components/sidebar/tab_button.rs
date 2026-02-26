use std::rc::Rc;

use gpui::{App, ElementId, Window, div, prelude::*};

use crate::pages::ActivePage;
use crate::theme::Colors;

#[derive(IntoElement)]
pub struct TabButton {
    page: ActivePage,
    label: &'static str,
    is_active: bool,
    on_click: Rc<dyn Fn(&ActivePage, &mut Window, &mut App) + 'static>,
}

impl TabButton {
    pub fn new(
        page: ActivePage,
        label: &'static str,
        is_active: bool,
        on_click: Rc<dyn Fn(&ActivePage, &mut Window, &mut App) + 'static>,
    ) -> Self {
        Self {
            page,
            label,
            is_active,
            on_click,
        }
    }
}

impl RenderOnce for TabButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let page = self.page;
        let on_click = self.on_click;
        let id = ElementId::Name(format!("tab-{}", self.label).into());

        let bg = if self.is_active {
            Colors::muted()
        } else {
            Colors::sidebar()
        };

        div()
            .id(id)
            .flex()
            .items_center()
            .px_3()
            .py_2()
            .bg(bg)
            .hover(move |style| style.bg(Colors::accent()))
            .cursor_pointer()
            .text_sm()
            .child(self.label)
            .on_click(move |_event, window, cx| {
                on_click(&page, window, cx);
            })
    }
}
