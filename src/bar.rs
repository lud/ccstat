use crate::config::{BLOCK, COLOR_EMPTY, COLOR_OVERLAP, COLOR_RESET, COLOR_TIME, COLOR_USAGE};

pub fn ansi_color_256(code: u8) -> String {
    format!("\x1b[38;5;{}m", code)
}

/// Single-color bar for the context window (blue fill, grey empty).
pub fn draw_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).floor() as usize;
    let mut bar = String::new();
    for i in 0..width {
        bar.push_str(&ansi_color_256(if i < filled {
            COLOR_USAGE
        } else {
            COLOR_EMPTY
        }));
        bar.push(BLOCK);
    }
    bar.push_str(COLOR_RESET);
    bar
}

/// Dual-progress bar for usage windows.
///
/// `usage_pct` — how much of the rate-limit has been consumed (0–100).
/// `time_pct`  — how far through the time window we are (0–100).
///
/// Four colors across `width` cells:
///   COLOR_OVERLAP (purple) — cell is within BOTH the usage and time regions
///   COLOR_USAGE   (blue)   — cell is in usage region only (usage ahead of time)
///   COLOR_TIME    (red)    — cell is in time region only  (time ahead of usage)
///   COLOR_EMPTY   (grey)   — cell is beyond both regions
///
/// Use `ansi_256(color)` per cell followed by `BLOCK`, then `COLOR_RESET` at the end.
pub fn draw_dual_bar(usage_pct: f64, time_pct: f64, width: usize) -> String {
    let usage_filled = ((usage_pct / 100.0) * width as f64).floor() as usize;
    let time_filled = ((time_pct / 100.0) * width as f64).floor() as usize;
    let mut bar = String::new();
    for i in 0..width {
        if i < usage_filled && i < time_filled {
            bar.push_str(&ansi_color_256(COLOR_OVERLAP));
        } else if i < usage_filled {
            bar.push_str(&ansi_color_256(COLOR_USAGE));
        } else if i < time_filled {
            bar.push_str(&ansi_color_256(COLOR_TIME));
        } else {
            bar.push_str(&ansi_color_256(COLOR_EMPTY));
        }
        bar.push(BLOCK);
    }
    bar.push_str(COLOR_RESET);
    bar
}
