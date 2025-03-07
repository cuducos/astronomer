use std::num::ParseIntError;
use wasm_bindgen::prelude::wasm_bindgen;

fn parse_rgb(hex: String) -> Result<(u8, u8, u8), ParseIntError> {
    Ok((
        u8::from_str_radix(&hex[1..3], 16)?,
        u8::from_str_radix(&hex[3..5], 16)?,
        u8::from_str_radix(&hex[5..7], 16)?,
    ))
}

#[wasm_bindgen]
pub fn color(hex: String, position: usize, total: usize) -> String {
    if total <= 1 || !hex.starts_with('#') || hex.len() != 7 {
        return hex;
    }
    parse_rgb(hex.clone())
        .map(|(r, g, b)| {
            let p = 1.0 - (position as f32) / ((total - 1) as f32);
            let a = 0.25 + p * 0.75;
            format!("rgba({}, {}, {}, {})", r, g, b, a)
        })
        .unwrap_or(hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_single_color() {
        assert_eq!(color("#ff5733".to_string(), 0, 1), "#ff5733");
        assert_eq!(color("#ff5733".to_string(), 0, 0), "#ff5733");
    }

    #[test]
    fn test_color_multiple_colors() {
        assert_eq!(color("#ff5733".to_string(), 0, 3), "rgba(255, 87, 51, 1)");
        assert_eq!(
            color("#ff5733".to_string(), 1, 3),
            "rgba(255, 87, 51, 0.625)"
        );
        assert_eq!(
            color("#ff5733".to_string(), 2, 3),
            "rgba(255, 87, 51, 0.25)"
        );
        assert_eq!(color("#ff5733".to_string(), 0, 10), "rgba(255, 87, 51, 1)");
        assert_eq!(
            color("#ff5733".to_string(), 9, 10),
            "rgba(255, 87, 51, 0.25)"
        );
    }

    #[test]
    fn test_color_with_invalid_hex() {
        assert_eq!(color("#zzzzzz".to_string(), 0, 1), "#zzzzzz");
    }
}
