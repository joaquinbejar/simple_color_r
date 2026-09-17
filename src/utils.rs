// utils.rs
use crate::color_map::initialize_color_map;
use crate::colors::Colors;

pub fn give_color(color: &Colors, text: &str, blink: bool) -> String {
    let color_map = initialize_color_map();
    let color_code = color_map.get(color).unwrap_or(&"");
    let blink_sequence = if blink { "\\033[5m" } else { "" };

    format!("{}{}{}\\033[0m", blink_sequence, color_code, text)
}
