use std::borrow::{Borrow, BorrowMut};
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex, MutexGuard};
use std::usize;

use crate::event::{Event, EventListener, EventPropagation};
use crate::kurbo::Point;
use crate::peniko::{Brush, Color, ColorStop, ColorStops, Extend, Gradient, GradientKind};
use crate::reactive::RwSignal;
use crate::style::{Background, CursorStyle, Transition};
use crate::taffy::AlignItems;
use crate::views::{
    container, dyn_container, empty, label, scroll, stack, static_label, tab, text_input, tooltip,
    virtual_stack, VirtualDirection, VirtualItemSize,
};
use crate::views::{h_stack, Decorators};
use crate::views::{svg, v_stack};
use crate::{views::button, IntoView};
use floem_reactive::{ReadSignal, SignalGet};

use crate::unit::{DurationUnitExt, UnitExt};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub fn rgb_to_wgpu(r: u8, g: u8, b: u8, a: f32) -> [f32; 4] {
    [
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a.clamp(0.0, 1.0),
    ]
}

static ICON_CACHE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn create_icon(name: &str) -> String {
    // Try to retrieve from cache first
    if let Some(icon) = ICON_CACHE.lock().unwrap().get(name) {
        return icon.clone();
    }

    // If not in cache, load and cache it
    let icon = match name {
        "plus" => include_str!("../assets/plus-thin.svg"),
        "minus" => include_str!("../assets/minus-thin.svg"),
        "windmill" => include_str!("../assets/windmill-thin.svg"),
        "gear" => include_str!("../assets/gear-six-thin.svg"),
        "brush" => include_str!("../assets/paint-brush-thin.svg"),
        "shapes" => include_str!("../assets/shapes-thin.svg"),
        "arrow-left" => include_str!("../assets/arrow-left-thin.svg"),
        "polygon" => include_str!("../assets/polygon-thin.svg"),
        "octagon" => include_str!("../assets/octagon-thin.svg"),
        "square" => include_str!("../assets/square-thin.svg"),
        "triangle" => include_str!("../assets/triangle-thin.svg"),
        "dot" => include_str!("../assets/dot-outline-thin.svg"),
        "dots-vertical" => include_str!("../assets/dots-three-outline-vertical-thin.svg"),
        "sphere" => include_str!("../assets/sphere-thin.svg"),
        "gizmo" => include_str!("../assets/vector-three-thin.svg"),
        "book" => include_str!("../assets/book-open-thin.svg"),
        "cube" => include_str!("../assets/cube-focus-thin.svg"),
        "faders" => include_str!("../assets/faders-thin.svg"),
        "map" => include_str!("../assets/map-trifold-thin.svg"),
        "panorama" => include_str!("../assets/panorama-thin.svg"),
        "speedometer" => include_str!("../assets/speedometer-thin.svg"),
        "motion-arrow" => include_str!("../assets/arrow-fat-lines-right-thin.svg"),
        "atom" => include_str!("../assets/atom-thin.svg"),
        "brain" => include_str!("../assets/brain-thin.svg"),
        "broadcast" => include_str!("../assets/broadcast-thin.svg"),
        "circles" => include_str!("../assets/circles-three-thin.svg"),
        "fast-forward" => include_str!("../assets/fast-forward-thin.svg"),
        "folder-plus" => include_str!("../assets/folder-plus-thin.svg"),
        "bone" => include_str!("../assets/bone-thin.svg"),
        "caret-down" => include_str!("../assets/caret-down-thin.svg"),
        "caret-right" => include_str!("../assets/caret-right-thin.svg"),
        "translate" => include_str!("../assets/arrows-out-cardinal-thin.svg"),
        "rotate" => include_str!("../assets/arrows-clockwise-thin.svg"),
        "scale" => include_str!("../assets/resize-thin.svg"),
        "image" => include_str!("../assets/image-thin.svg"),
        "text" => include_str!("../assets/text-t-thin.svg"),
        "video" => include_str!("../assets/video-thin.svg"),
        "copy" => include_str!("../assets/copy-thin.svg"),
        "trash" => include_str!("../assets/trash-thin.svg"),
        "x" => include_str!("../assets/x-thin.svg"),
        _ => "",
    };

    // Store in cache
    ICON_CACHE
        .lock()
        .unwrap()
        .insert(name.to_string(), icon.to_string());

    icon.to_string()
}

pub fn small_button(
    text: &'static str,
    icon_name: &'static str,
    action: impl FnMut(&Event) + 'static,
    active: RwSignal<bool>,
) -> impl IntoView {
    button(
        h_stack((
            svg(create_icon(icon_name)).style(|s| s.width(24).height(24).color(Color::BLACK)),
            if text.len() > 0 {
                label(move || text).style(|s| s.margin_left(4.0))
            } else {
                label(move || text)
            },
        ))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .on_click_stop(action)
    .style(move |s| {
        s.height(28)
            .justify_center()
            .align_items(AlignItems::Center)
            .background(if active.get() {
                Color::LIGHT_GRAY
            } else {
                Color::WHITE
            })
            .border(0)
            // .border_color(Color::BLACK)
            .border_radius(15)
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
            .z_index(20)
    })
}

pub fn simple_button(text: String, action: impl FnMut(&Event) + 'static) -> impl IntoView {
    button(
        h_stack((if text.len() > 0 {
            label(move || text.clone()).style(|s| s.margin_left(4.0))
        } else {
            label(move || text.clone())
        },))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .on_click_stop(action)
    .style(move |s| {
        s.height(28)
            .justify_center()
            .align_items(AlignItems::Center)
            .border(0)
            // .border_color(Color::BLACK)
            .border_radius(15)
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
            .z_index(20)
    })
}

pub fn icon_button(
    icon_name: &str,
    tooltip_text: String,
    action: impl FnMut(&Event) + 'static,
) -> impl IntoView {
    tooltip(
        button(
            h_stack((
                svg(create_icon(icon_name)).style(|s| s.width(20).height(20).color(Color::BLACK)),
            ))
            .style(|s| s.justify_center().align_items(AlignItems::Center)),
        )
        .on_click_stop(action)
        .style(move |s| {
            s.height(28)
                .width(28.0)
                .justify_center()
                .align_items(AlignItems::Center)
                .border(0)
                // .border_color(Color::BLACK)
                .border_radius(15)
                .transition(Background, Transition::ease_in_out(200.millis()))
                .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
                .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
                .z_index(20)
        }),
        move || static_label(&tooltip_text),
    )
}

pub fn toggle_button(
    text: &'static str,
    icon_name: &'static str,
    this_toggle: String,
    action: impl FnMut(&Event) + 'static,
    active: RwSignal<String>,
) -> impl IntoView {
    button(
        h_stack((
            svg(create_icon(icon_name)).style(|s| s.width(24).height(24).color(Color::BLACK)),
            if text.len() > 0 {
                label(move || text).style(|s| s.margin_left(4.0))
            } else {
                label(move || text)
            },
        ))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .on_click_stop(action)
    .style(move |s| {
        s.height(28)
            .justify_center()
            .align_items(AlignItems::Center)
            .background(Color::WHITE)
            .border(1)
            .border_color(Color::DARK_GRAY)
            .border_radius(15)
            .padding(4.0)
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
            .apply_if(this_toggle == active.get(), |s| {
                s.background(Color::GRAY).color(Color::WHITE_SMOKE)
            })
    })
}

pub fn success_button(
    text: &'static str,
    icon_name: &'static str,
    action: Option<impl FnMut() + 'static>,
    active: bool,
) -> impl IntoView {
    // Radial gradient with different start and end circles
    let green = rgb_to_wgpu(153, 199, 162, 1.0);
    let yellow = rgb_to_wgpu(200, 204, 124, 1.0);
    // let green = (153, 199, 162, 1.0);
    // let yellow = (200, 204, 124, 1.0);

    // Linear gradient from left to right
    let gradient = Gradient {
        kind: GradientKind::Linear {
            start: Point::new(50.0, 0.0), // Start further left
            end: Point::new(200.0, 50.0), // End further right to allow more space
        },
        extend: Extend::Pad,
        stops: ColorStops::from_vec(vec![
            ColorStop {
                offset: 0.5,
                color: Color::rgb(green[0] as f64, green[1] as f64, green[2] as f64),
            },
            ColorStop {
                offset: 1.0,
                color: Color::rgb(yellow[0] as f64, yellow[1] as f64, yellow[2] as f64),
            },
        ]),
    };

    button(
        v_stack((
            svg(create_icon(icon_name)).style(|s| s.width(24).height(24).color(Color::BLACK)),
            label(move || text).style(|s| s.margin_top(4.0)),
        ))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .action(action)
    .style(move |s| {
        s.height(100)
            .width(100.0)
            .justify_center()
            .align_items(AlignItems::Center)
            .background(
                Gradient::new_linear(
                    crate::kurbo::Point::new(0.0, 0.0),
                    crate::kurbo::Point::new(100.0, 100.0),
                )
                .with_stops([
                    (0.0, Color::rgba(0.2, 0.4, 0.6, 1.0)), // start color
                    (1.0, Color::rgba(0.4, 0.6, 0.8, 1.0)), // end color
                ]),
            )
            .border(0)
            // .border_color(Color::BLACK)
            .border_radius(15)
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
            .z_index(20)
    })
}

pub fn nav_button(
    text: &'static str,
    icon_name: &'static str,
    action: Option<impl FnMut() + 'static>,
    active: bool,
) -> impl IntoView {
    button(
        v_stack((
            svg(create_icon(icon_name)).style(|s| s.width(30).height(30)),
            label(move || text).style(|s| s.margin_top(4.0)),
        ))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .action(action)
    .style(move |s| {
        s.width(70)
            .height(70)
            .justify_center()
            .align_items(AlignItems::Center)
            .border(0)
            .border_radius(15)
            .box_shadow_blur(15)
            .box_shadow_spread(4)
            .box_shadow_color(Color::rgba(0.0, 0.0, 0.0, 0.36))
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
    })
}

pub fn option_button(
    text: &'static str,
    icon_name: &'static str,
    action: Option<impl FnMut() + 'static>,
    active: bool,
) -> impl IntoView {
    button(
        v_stack((
            svg(create_icon(icon_name)).style(|s| s.width(24).height(24)),
            label(move || text).style(|s| s.margin_top(4.0).font_size(9.0)),
        ))
        .style(|s| s.justify_center().align_items(AlignItems::Center)),
    )
    .action(action)
    .style(move |s| {
        s.width(60)
            .height(60)
            .justify_center()
            .align_items(AlignItems::Center)
            .border(1.0)
            .border_color(Color::GRAY)
            .border_radius(15)
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
    })
}

// pub fn option_button_once(
//     text: &'static str,
//     icon_name: &'static str,
//     action: impl FnOnce() + 'static,
//     active: bool,
// ) -> impl IntoView {
//     button(
//         v_stack((
//             svg(create_icon(icon_name)).style(|s| s.width(24).height(24)),
//             label(move || text).style(|s| s.margin_top(4.0).font_size(9.0)),
//         ))
//         .style(|s| s.justify_center().align_items(AlignItems::Center)),
//     )
//     // .action(action)
//     .on_click(move |_| {
//         // let action = action.expect("Couldn't get action");

//         action();

//         EventPropagation::Stop
//     })
//     .style(move |s| {
//         s.width(60)
//             .height(60)
//             .justify_center()
//             .align_items(AlignItems::Center)
//             .border(1.0)
//             .border_color(Color::GRAY)
//             .border_radius(15)
//             .transition(Background, Transition::ease_in_out(200.millis()))
//             .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
//             .hover(|s| s.background(Color::LIGHT_GRAY).cursor(CursorStyle::Pointer))
//     })
// }

// pub fn layer_button(layer_name: String, icon_name: &'static str) -> impl IntoView {
//     h_stack((
//         svg(create_icon(icon_name))
//             .style(|s| s.width(24).height(24).color(Color::BLACK))
//             .style(|s| s.margin_right(4.0)),
//         label(move || layer_name.to_string()),
//     ))
//     .style(|s| {
//         s.align_items(AlignItems::Center)
//             .padding_vert(8)
//             .background(Color::rgb(255.0, 239.0, 194.0))
//             .border_bottom(1)
//             .border_color(Color::rgb(200.0, 200.0, 200.0))
//             .border_radius(4)
//             .hover(|s| s.background(Color::rgb(222.0, 206.0, 160.0)))
//             .active(|s| s.background(Color::rgb(237.0, 218.0, 164.0)))
//     })
//     .on_click(|_| {
//         println!("Layer selected");
//         EventPropagation::Stop
//     })
// }

pub fn tab_button(
    text: &'static str,
    // icon_name: &'static str,
    action: Option<impl FnMut() + 'static>,
    this_tab: usize,
    active: ReadSignal<usize>,
) -> impl IntoView {
    button(
        v_stack((
            // svg(create_icon(icon_name)).style(|s| s.width(30).height(30)),
            label(move || text).style(|s| s.margin_top(4.0)),
        ))
        .style(|s| {
            s.color(Color::WHITE)
                .justify_center()
                .align_items(AlignItems::Center)
        }),
    )
    .action(action)
    .style(move |s| {
        s.width(90)
            .height(30)
            .justify_center()
            .align_items(AlignItems::Center)
            .border(0)
            .background(Color::DARK_GRAY)
            .border_radius(5.0)
            .apply_if(this_tab == active.get(), |s| s.background(Color::BLACK))
            .transition(Background, Transition::ease_in_out(200.millis()))
            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
            .hover(|s| s.background(Color::DARK_GRAY).cursor(CursorStyle::Pointer))
            .margin_right(4.0)
    })
}
