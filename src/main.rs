#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};
use gpui_platform::application;

struct HelloWorld {
    text: SharedString,
}

fn color_swatch(color: gpui::Hsla, border: gpui::Hsla) -> impl IntoElement {
    div()
        .size_8()
        .bg(color)
        .border_1()
        .border_dashed()
        .rounded_md()
        .border_color(border)
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x405050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x00eeee))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(color_swatch(gpui::red(), gpui::white()))
                    .child(color_swatch(gpui::green(), gpui::white()))
                    .child(color_swatch(gpui::blue(), gpui::white()))
                    .child(color_swatch(gpui::yellow(), gpui::white()))
                    .child(color_swatch(gpui::black(), gpui::white()))
                    .child(color_swatch(gpui::white(), gpui::black())),
            )
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}
