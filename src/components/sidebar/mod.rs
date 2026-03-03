mod tab_button;

use std::rc::Rc;

use gpui::{App, Window, div, prelude::*};

use crate::app::LoginStatus;
use crate::auth::types::PlayerProfile;
use crate::components::user_profile::UserProfile;
use crate::pages::ActivePage;
use crate::theme::Colors;
use tab_button::TabButton;

#[derive(IntoElement)]
pub struct Sidebar {
    active_page: ActivePage,
    profile: Option<PlayerProfile>,
    login_status: LoginStatus,
    on_tab_click: Rc<dyn Fn(&ActivePage, &mut Window, &mut App) + 'static>,
    on_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    on_cancel_login_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    on_logout_click: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl Sidebar {
    pub fn new(
        active_page: ActivePage,
        profile: Option<PlayerProfile>,
        login_status: LoginStatus,
        cx: &mut gpui::Context<crate::app::App>,
    ) -> Self {
        let on_tab_click = cx.listener(|app, page: &ActivePage, _window, cx| {
            app.active_page = *page;
            cx.notify();
        });

        let on_login_click = cx.listener(|app, _: &(), window, cx| {
            app.start_login(window, cx);
        });

        let on_cancel_login_click = cx.listener(|app, _: &(), window, cx| {
            app.cancel_login(window, cx);
        });

        let on_logout_click = cx.listener(|app, _: &(), window, cx| {
            app.logout(window, cx);
        });

        Self {
            active_page,
            profile,
            login_status,
            on_tab_click: Rc::new(on_tab_click),
            on_login_click: Rc::new(move |window, cx| on_login_click(&(), window, cx)),
            on_cancel_login_click: Rc::new(move |window, cx| on_cancel_login_click(&(), window, cx)),
            on_logout_click: Rc::new(move |window, cx| on_logout_click(&(), window, cx)),
        }
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let tabs: &[(ActivePage, &str)] = &[
            (ActivePage::Instances, "Instances"),
            (ActivePage::Create, "Create"),
            (ActivePage::Settings, "Settings"),
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
                    .child(UserProfile::new(
                        self.profile,
                        self.login_status,
                        self.on_login_click,
                        self.on_cancel_login_click,
                        self.on_logout_click,
                    )),
            )
    }
}
