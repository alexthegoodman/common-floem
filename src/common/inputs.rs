use crate::peniko::Color;
use crate::style::Style;

pub fn input_styles(s: Style) -> Style {
    s.width_full()
        .border(1)
        .border_color(Color::GRAY)
        .border_radius(4)
        .padding_horiz(5)
        .padding_vert(3)
}
