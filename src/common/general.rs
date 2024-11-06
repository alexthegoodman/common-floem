use floem_reactive::create_rw_signal;

use crate::peniko::Color;
use crate::style::Style;
use crate::view::View;
use crate::views::container;
use crate::views::{h_stack, label};
use crate::{
    reactive::create_signal,
    unit::UnitExt,
    views::{img, svg, Decorators},
    IntoView,
};
use floem_reactive::SignalGet;
use floem_reactive::SignalUpdate;
use std::fs;

pub fn card_styles(s: Style) -> Style {
    s.padding(20)
        .background(Color::rgba(240.0, 240.0, 240.0, 255.0))
        .border_radius(15)
        .box_shadow_blur(15)
        .box_shadow_spread(4)
        .box_shadow_color(Color::rgba(0.0, 0.0, 0.0, 0.36))
}

#[derive(Clone, Copy)]
pub enum AlertVariant {
    Success,
    Info,
    Error,
    Warning,
}

impl AlertVariant {
    fn get_colors(&self) -> (Color, Color) {
        match self {
            AlertVariant::Success => (
                Color::rgb8(240, 253, 244), // bg-green-50
                Color::rgb8(22, 163, 74),   // text-green-600
            ),
            AlertVariant::Info => (
                Color::rgb8(239, 246, 255), // bg-blue-50
                Color::rgb8(37, 99, 235),   // text-blue-600
            ),
            AlertVariant::Error => (
                Color::rgb8(254, 242, 242), // bg-red-50
                Color::rgb8(220, 38, 38),   // text-red-600
            ),
            AlertVariant::Warning => (
                Color::rgb8(254, 252, 232), // bg-yellow-50
                Color::rgb8(202, 138, 4),   // text-yellow-600
            ),
        }
    }
}

pub fn alert(variant: AlertVariant, message: String) -> impl View {
    let (bg_color, text_color) = variant.get_colors();

    container((label(move || message.clone()).style(move |s| s.color(text_color).font_size(14.0))))
        .style(move |s| {
            s.padding(12.0)
                .border_radius(6.0)
                .background(bg_color)
                .width_full()
        })
}
