//! Algorithmic formatters for the `color` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ── Static color name table (same order as Python `all_colors`) ───────────────
static ALL_COLOR_NAMES: &[&str] = &[
    "AliceBlue",
    "AntiqueWhite",
    "Aqua",
    "Aquamarine",
    "Azure",
    "Beige",
    "Bisque",
    "Black",
    "BlanchedAlmond",
    "Blue",
    "BlueViolet",
    "Brown",
    "BurlyWood",
    "CadetBlue",
    "Chartreuse",
    "Chocolate",
    "Coral",
    "CornflowerBlue",
    "Cornsilk",
    "Crimson",
    "Cyan",
    "DarkBlue",
    "DarkCyan",
    "DarkGoldenRod",
    "DarkGray",
    "DarkGreen",
    "DarkKhaki",
    "DarkMagenta",
    "DarkOliveGreen",
    "DarkOrange",
    "DarkOrchid",
    "DarkRed",
    "DarkSalmon",
    "DarkSeaGreen",
    "DarkSlateBlue",
    "DarkSlateGray",
    "DarkTurquoise",
    "DarkViolet",
    "DeepPink",
    "DeepSkyBlue",
    "DimGray",
    "DodgerBlue",
    "FireBrick",
    "FloralWhite",
    "ForestGreen",
    "Fuchsia",
    "Gainsboro",
    "GhostWhite",
    "Gold",
    "GoldenRod",
    "Gray",
    "Green",
    "GreenYellow",
    "HoneyDew",
    "HotPink",
    "IndianRed",
    "Indigo",
    "Ivory",
    "Khaki",
    "Lavender",
    "LavenderBlush",
    "LawnGreen",
    "LemonChiffon",
    "LightBlue",
    "LightCoral",
    "LightCyan",
    "LightGoldenRodYellow",
    "LightGray",
    "LightGreen",
    "LightPink",
    "LightSalmon",
    "LightSeaGreen",
    "LightSkyBlue",
    "LightSlateGray",
    "LightSteelBlue",
    "LightYellow",
    "Lime",
    "LimeGreen",
    "Linen",
    "Magenta",
    "Maroon",
    "MediumAquaMarine",
    "MediumBlue",
    "MediumOrchid",
    "MediumPurple",
    "MediumSeaGreen",
    "MediumSlateBlue",
    "MediumSpringGreen",
    "MediumTurquoise",
    "MediumVioletRed",
    "MidnightBlue",
    "MintCream",
    "MistyRose",
    "Moccasin",
    "NavajoWhite",
    "Navy",
    "OldLace",
    "Olive",
    "OliveDrab",
    "Orange",
    "OrangeRed",
    "Orchid",
    "PaleGoldenRod",
    "PaleGreen",
    "PaleTurquoise",
    "PaleVioletRed",
    "PapayaWhip",
    "PeachPuff",
    "Peru",
    "Pink",
    "Plum",
    "PowderBlue",
    "Purple",
    "Red",
    "RosyBrown",
    "RoyalBlue",
    "SaddleBrown",
    "Salmon",
    "SandyBrown",
    "SeaGreen",
    "SeaShell",
    "Sienna",
    "Silver",
    "SkyBlue",
    "SlateBlue",
    "SlateGray",
    "Snow",
    "SpringGreen",
    "SteelBlue",
    "Tan",
    "Teal",
    "Thistle",
    "Tomato",
    "Turquoise",
    "Violet",
    "Wheat",
    "White",
    "WhiteSmoke",
    "Yellow",
    "YellowGreen",
];

static SAFE_COLORS: &[&str] = &[
    "black", "maroon", "green", "navy", "olive", "purple", "teal", "lime", "blue", "silver",
    "gray", "yellow", "fuchsia", "aqua", "white",
];

// ── Color map for human-friendly generation (mirrors color.py COLOR_MAP) ──────
// Each entry: (name, hue_min, hue_max, lower_bounds: &[(s, v)])
struct ColorInfo {
    hue_min: i32,
    hue_max: i32,
    lower_bounds: &'static [(i32, i32)],
}

static COLOR_MAP: &[(&str, ColorInfo)] = &[
    (
        "monochrome",
        ColorInfo {
            hue_min: 0,
            hue_max: 0,
            lower_bounds: &[(0, 0), (100, 0)],
        },
    ),
    (
        "red",
        ColorInfo {
            hue_min: -26,
            hue_max: 18,
            lower_bounds: &[
                (20, 100),
                (30, 92),
                (40, 89),
                (50, 85),
                (60, 78),
                (70, 70),
                (80, 60),
                (90, 55),
                (100, 50),
            ],
        },
    ),
    (
        "orange",
        ColorInfo {
            hue_min: 19,
            hue_max: 46,
            lower_bounds: &[
                (20, 100),
                (30, 93),
                (40, 88),
                (50, 86),
                (60, 85),
                (70, 70),
                (100, 70),
            ],
        },
    ),
    (
        "yellow",
        ColorInfo {
            hue_min: 47,
            hue_max: 62,
            lower_bounds: &[
                (25, 100),
                (40, 94),
                (50, 89),
                (60, 86),
                (70, 84),
                (80, 82),
                (90, 80),
                (100, 75),
            ],
        },
    ),
    (
        "green",
        ColorInfo {
            hue_min: 63,
            hue_max: 178,
            lower_bounds: &[
                (30, 100),
                (40, 90),
                (50, 85),
                (60, 81),
                (70, 74),
                (80, 64),
                (90, 50),
                (100, 40),
            ],
        },
    ),
    (
        "blue",
        ColorInfo {
            hue_min: 179,
            hue_max: 257,
            lower_bounds: &[
                (20, 100),
                (30, 86),
                (40, 80),
                (50, 74),
                (60, 60),
                (70, 52),
                (80, 44),
                (90, 39),
                (100, 35),
            ],
        },
    ),
    (
        "purple",
        ColorInfo {
            hue_min: 258,
            hue_max: 282,
            lower_bounds: &[
                (20, 100),
                (30, 87),
                (40, 79),
                (50, 70),
                (60, 65),
                (70, 59),
                (80, 52),
                (90, 45),
                (100, 42),
            ],
        },
    ),
    (
        "pink",
        ColorInfo {
            hue_min: 283,
            hue_max: 334,
            lower_bounds: &[
                (20, 100),
                (30, 90),
                (40, 86),
                (60, 84),
                (80, 80),
                (90, 75),
                (100, 73),
            ],
        },
    ),
];

// ── HSV helpers ───────────────────────────────────────────────────────────────

fn get_color_info(hue: i32) -> Option<&'static ColorInfo> {
    let h = if (334..=360).contains(&hue) {
        hue - 360
    } else {
        hue
    };
    COLOR_MAP
        .iter()
        .map(|(_, info)| info)
        .find(|&info| info.hue_min <= h && h <= info.hue_max)
        .map(|v| v as _)
}

fn get_minimum_brightness(hue: i32, s: i32) -> i32 {
    if let Some(info) = get_color_info(hue) {
        let lb = info.lower_bounds;
        for i in 0..lb.len().saturating_sub(1) {
            let (s1, v1) = lb[i];
            let (s2, v2) = lb[i + 1];
            if s1 <= s && s <= s2 {
                let m = (v2 - v1) as f64 / (s2 - s1) as f64;
                let b = v1 as f64 - m * s1 as f64;
                return (m * s as f64 + b) as i32;
            }
        }
    }
    0
}

fn get_saturation_range(hue: i32) -> (i32, i32) {
    if let Some(info) = get_color_info(hue) {
        let sats: Vec<i32> = info.lower_bounds.iter().map(|(s, _)| *s).collect();
        let lo = *sats.iter().min().unwrap_or(&0);
        let hi = *sats.iter().max().unwrap_or(&100);
        return (lo, hi);
    }
    (0, 100)
}

/// Pick a random hue from [min, max] inclusive.
fn random_within(f: &Faker, lo: i32, hi: i32) -> i32 {
    f.rng.random_int(lo as i64, hi as i64, 1) as i32
}

/// Generate HSV triple using the RandomColor algorithm with no specific hue/luminosity.
fn generate_hsv(f: &Faker) -> (i32, i32, i32) {
    // Pick hue from 0..360
    let mut h = random_within(f, 0, 360);
    if h < 0 {
        h += 360;
    }

    // Pick saturation
    let (s_min, s_max) = get_saturation_range(h);
    let s = random_within(f, s_min, s_max);

    // Pick brightness
    let b_min = get_minimum_brightness(h, s);
    let b_max = 100;
    let v = random_within(f, b_min, b_max);

    (h, s, v)
}

/// Convert HSV (h:0-360, s:0-100, v:0-100) to RGB (0-255 each).
fn hsv_to_rgb(h: i32, s: i32, v: i32) -> (u8, u8, u8) {
    let h = h.max(1).min(359) as f64;
    let s = s as f64 / 100.0;
    let v = v as f64 / 100.0;

    // Standard HSV->RGB conversion
    let hh = h / 60.0;
    let i = hh.floor() as i32;
    let ff = hh - hh.floor();
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * ff);
    let t = v * (1.0 - s * (1.0 - ff));

    let (r, g, b) = match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

/// Convert HSV to HSL.
fn hsv_to_hsl(h: i32, s: i32, v: i32) -> (i32, i32, i32) {
    let s_ = s as f64 / 100.0;
    let v_ = v as f64 / 100.0;
    let l = 0.5 * v_ * (2.0 - s_);
    let denom = 1.0 - (2.0 * l - 1.0).abs();
    let s_l = if denom == 0.0 { 0.0 } else { v_ * s_ / denom };
    (h, (s_l * 100.0) as i32, (l * 100.0) as i32)
}

// ── Formatter functions ───────────────────────────────────────────────────────

fn color_name(f: &Faker, _locale: &str) -> String {
    let idx = f.rng.below(ALL_COLOR_NAMES.len());
    ALL_COLOR_NAMES[idx].to_string()
}

fn safe_color_name(f: &Faker, _locale: &str) -> String {
    let idx = f.rng.below(SAFE_COLORS.len());
    SAFE_COLORS[idx].to_string()
}

fn hex_color(f: &Faker, _locale: &str) -> String {
    let v = f.rng.random_int(1, 16_777_215, 1) as u32;
    format!("#{:06x}", v)
}

fn safe_hex_color(f: &Faker, _locale: &str) -> String {
    let r = (f.rng.random_int(0, 15, 1) as u32) * 17;
    let g = (f.rng.random_int(0, 15, 1) as u32) * 17;
    let b = (f.rng.random_int(0, 15, 1) as u32) * 17;
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn rgb_color(f: &Faker, _locale: &str) -> String {
    let r = f.rng.random_int(0, 255, 1);
    let g = f.rng.random_int(0, 255, 1);
    let b = f.rng.random_int(0, 255, 1);
    format!("{},{},{}", r, g, b)
}

fn rgb_css_color(f: &Faker, _locale: &str) -> String {
    let r = f.rng.random_int(0, 255, 1);
    let g = f.rng.random_int(0, 255, 1);
    let b = f.rng.random_int(0, 255, 1);
    format!("rgb({},{},{})", r, g, b)
}

/// `color()` with default args (hex format, no specific hue/luminosity).
fn color(f: &Faker, _locale: &str) -> String {
    let (h, s, v) = generate_hsv(f);
    let (r, g, b) = hsv_to_rgb(h, s, v);
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn color_hsv(f: &Faker, _locale: &str) -> String {
    let (h, s, v) = generate_hsv(f);
    format!("hsv({}, {}, {})", h, s, v)
}

fn color_hsl(f: &Faker, _locale: &str) -> String {
    let (h, s, v) = generate_hsv(f);
    let (hh, ss, ll) = hsv_to_hsl(h, s, v);
    format!("hsl({}, {}, {})", hh, ss, ll)
}

fn color_rgb(f: &Faker, _locale: &str) -> String {
    let (h, s, v) = generate_hsv(f);
    let (r, g, b) = hsv_to_rgb(h, s, v);
    format!("({}, {}, {})", r, g, b)
}

fn color_rgb_float(f: &Faker, _locale: &str) -> String {
    let (h, s, v) = generate_hsv(f);
    let (r, g, b) = hsv_to_rgb(h, s, v);
    format!(
        "({:.4}, {:.4}, {:.4})",
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0
    )
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "color_name" => color_name(f, locale),
        "safe_color_name" => safe_color_name(f, locale),
        "hex_color" => hex_color(f, locale),
        "safe_hex_color" => safe_hex_color(f, locale),
        "rgb_color" => rgb_color(f, locale),
        "rgb_css_color" => rgb_css_color(f, locale),
        "color" => color(f, locale),
        "color_hsv" => color_hsv(f, locale),
        "color_hsl" => color_hsl(f, locale),
        "color_rgb" => color_rgb(f, locale),
        "color_rgb_float" => color_rgb_float(f, locale),
        _ => return None,
    })
}
