use gpui::{
    AnyElement, Context, DispatchPhase, Entity, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, Pixels, ScrollHandle, Window, canvas, div, fill, point, prelude::*, px,
};

use crate::components::sidebar::Sidebar;
use crate::pages::{ActivePage, create::CreatePage, instances::InstancesPage, settings::SettingsPage};
use crate::theme::Colors;

const SCROLLBAR_WIDTH: f32 = 6.0;
const SCROLLBAR_MIN_THUMB: f32 = 20.0;

pub struct App {
    pub(crate) active_page: ActivePage,
    scroll_handle: ScrollHandle,
    is_dragging_scrollbar: bool,
    is_scrollbar_hovered: bool,
    drag_start_y: Pixels,
    drag_start_scroll_offset: Pixels,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_page: ActivePage::Instances,
            scroll_handle: ScrollHandle::new(),
            is_dragging_scrollbar: false,
            is_scrollbar_hovered: false,
            drag_start_y: px(0.),
            drag_start_scroll_offset: px(0.),
        }
    }

    fn content_height(&self) -> Pixels {
        self.scroll_handle.max_offset().height + self.scroll_handle.bounds().size.height
    }

    fn track_height(&self) -> Pixels {
        self.scroll_handle.bounds().size.height
    }

    fn thumb_height(&self) -> Pixels {
        let track = self.track_height();
        let content = self.content_height();
        if content <= track {
            return px(0.);
        }
        (track / content * track).max(px(SCROLLBAR_MIN_THUMB))
    }

    fn max_scroll(&self) -> Pixels {
        self.content_height() - self.track_height()
    }

    fn on_scrollbar_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let content = self.content_height();
        let track = self.track_height();
        if content <= track {
            return;
        }

        self.is_dragging_scrollbar = true;
        self.drag_start_y = event.position.y;
        self.drag_start_scroll_offset = -self.scroll_handle.offset().y;
        cx.notify();
    }

    fn apply_drag(&mut self, mouse_y: Pixels) {
        let track = self.track_height();
        let thumb = self.thumb_height();
        let max_scroll = self.max_scroll();
        let draggable_range = track - thumb;

        if draggable_range <= px(0.) {
            return;
        }

        let delta_y = mouse_y - self.drag_start_y;
        let scroll_delta = delta_y / draggable_range * max_scroll;
        let new_offset = (self.drag_start_scroll_offset + scroll_delta)
            .max(px(0.))
            .min(max_scroll);

        self.scroll_handle.set_offset(point(px(0.), -new_offset));
    }

    fn render_content(&self) -> AnyElement {
        match self.active_page {
            ActivePage::Instances => InstancesPage::render().into_any_element(),
            ActivePage::Create => CreatePage::render().into_any_element(),
            ActivePage::Settings => SettingsPage::render().into_any_element(),
            _ => div()
                .flex()
                .flex_col()
                .size_full()
                .gap_2()
                .child(
                    div()
                        .text_xl()
                        .text_color(Colors::foreground())
                        .child(format!("{:?}", self.active_page)),
                )
                .child(
                    div()
                        .text_color(Colors::muted_foreground())
                        .text_sm()
                        .child("Temporary tab for scroll testing."),
                )
                .into_any_element(),
        }
    }

    fn render_scrollbar(&self, app_entity: Entity<Self>) -> impl IntoElement {
        let scroll_handle = self.scroll_handle.clone();
        let is_dragging = self.is_dragging_scrollbar;
        let thumb_color = if self.is_scrollbar_hovered || self.is_dragging_scrollbar {
            Colors::muted()
        } else {
            Colors::accent()
        };

        canvas(
            move |bounds, _window, _cx| (bounds, scroll_handle),
            move |_bounds, (bounds, scroll_handle), window, _cx| {
                // Register window-level drag listeners during paint phase
                if is_dragging {
                    let app_entity_move = app_entity.clone();
                    window.on_mouse_event({
                        let app_entity = app_entity.clone();
                        move |event: &MouseMoveEvent, phase, _window, cx| {
                            if phase != DispatchPhase::Capture || !event.dragging() {
                                return;
                            }
                            app_entity.update(cx, |app, cx| {
                                app.apply_drag(event.position.y);
                                cx.notify();
                            });
                            cx.stop_propagation();
                        }
                    });

                    window.on_mouse_event(move |_event: &MouseUpEvent, phase, _window, cx| {
                        if phase != DispatchPhase::Capture {
                            return;
                        }
                        app_entity_move.update(cx, |app, cx| {
                            app.is_dragging_scrollbar = false;
                            cx.notify();
                        });
                        cx.stop_propagation();
                    });
                }

                // Paint the thumb
                let max_offset = scroll_handle.max_offset();
                let content_height = max_offset.height + bounds.size.height;

                if content_height <= bounds.size.height {
                    return;
                }

                let offset_y = -scroll_handle.offset().y;
                let track_height = bounds.size.height;
                let thumb_height =
                    (track_height / content_height * track_height).max(px(SCROLLBAR_MIN_THUMB));
                let max_scroll = content_height - bounds.size.height;
                let thumb_y = offset_y / max_scroll * (track_height - thumb_height);

                let thumb_bounds = gpui::Bounds {
                    origin: point(bounds.origin.x, bounds.origin.y + thumb_y),
                    size: gpui::size(px(SCROLLBAR_WIDTH), thumb_height),
                };

                window.paint_quad(fill(thumb_bounds, thumb_color));
            },
        )
        .size_full()
    }
}

impl Render for App {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_entity = cx.entity().clone();
        let app_entity_hover = cx.entity().clone();

        div()
            .id("app-root")
            .flex()
            .flex_row()
            .size_full()
            .bg(Colors::background())
            .text_color(Colors::foreground())
            .child(Sidebar::new(self.active_page, cx))
            .child(
                div()
                    .flex_1()
                    .relative()
                    .child(
                        div()
                            .id("content-area")
                            .size_full()
                            .p_4()
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .child(self.render_content()),
                    )
                    .child(
                        div()
                            .id("scrollbar")
                            .absolute()
                            .top_0()
                            .right_0()
                            .bottom_0()
                            .w(px(SCROLLBAR_WIDTH))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(Self::on_scrollbar_mouse_down),
                            )
                            .on_hover(move |hovered, _window, cx| {
                                app_entity_hover.update(cx, |app, cx| {
                                    if app.is_scrollbar_hovered != *hovered {
                                        app.is_scrollbar_hovered = *hovered;
                                        cx.notify();
                                    }
                                });
                            })
                            .child(self.render_scrollbar(app_entity)),
                    ),
            )
    }
}
