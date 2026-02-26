mod tab_button;

use std::rc::Rc;

use gpui::{App, Window, div, prelude::*};

use crate::components::user_profile::UserProfile;
use crate::pages::ActivePage;
use crate::theme::Colors;
use tab_button::TabButton;

#[derive(IntoElement)]
pub struct Sidebar {
    active_page: ActivePage,
    on_tab_click: Rc<dyn Fn(&ActivePage, &mut Window, &mut App) + 'static>,
}

impl Sidebar {
    pub fn new(
        active_page: ActivePage,
        cx: &mut gpui::Context<crate::app::App>,
    ) -> Self {
        let on_tab_click = cx.listener(|app, page: &ActivePage, _window, cx| {
            app.active_page = *page;
            cx.notify();
        });

        Self {
            active_page,
            on_tab_click: Rc::new(on_tab_click),
        }
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let tabs: &[(ActivePage, &str)] = &[
            (ActivePage::Instances, "Instances"),
            (ActivePage::Create, "Create"),
            (ActivePage::Settings, "Settings"),
            // Temporary tabs for scroll testing
            (ActivePage::TempTab1, "Temp Tab 1"),
            (ActivePage::TempTab2, "Temp Tab 2"),
            (ActivePage::TempTab3, "Temp Tab 3"),
            (ActivePage::TempTab4, "Temp Tab 4"),
            (ActivePage::TempTab5, "Temp Tab 5"),
            (ActivePage::TempTab6, "Temp Tab 6"),
            (ActivePage::TempTab7, "Temp Tab 7"),
            (ActivePage::TempTab8, "Temp Tab 8"),
            (ActivePage::TempTab9, "Temp Tab 9"),
            (ActivePage::TempTab10, "Temp Tab 10"),
            (ActivePage::TempTab11, "Temp Tab 11"),
            (ActivePage::TempTab12, "Temp Tab 12"),
        ];

        div()
            .flex()
            .flex_col()
            .justify_between()
            .w(gpui::px(200.))
            .h_full()
            .bg(Colors::sidebar())
            .border_r_1()
            .border_color(Colors::border())
            .child(
                div()
                    .id("sidebar-tabs")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .children(tabs.iter().map(|(page, label)| {
                        let is_active = self.active_page == *page;
                        TabButton::new(*page, label, is_active, self.on_tab_click.clone())
                    })),
            )
            .child(
                div()
                    .flex_none()
                    .border_t_1()
                    .border_color(Colors::border())
                    .child(UserProfile::new(None)),
            )
    }
}
