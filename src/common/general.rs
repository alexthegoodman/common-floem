use floem_reactive::create_rw_signal;

use crate::peniko::Color;
use crate::style::Style;
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
