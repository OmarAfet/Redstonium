mod app;
mod components;
mod pages;
mod theme;

use app::App as RootApp;
use gpui::{App, AppContext as _, Bounds, SharedString, TitlebarOptions, WindowBounds, WindowOptions, px, size};
use gpui_platform::application;

const APP_NAME: &str = "Redstonium";

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(600.)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from(APP_NAME)),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| RootApp::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
