use crate::config::{BLOCK, COLOR_EMPTY, COLOR_OVERLAP, COLOR_TIME, COLOR_USAGE, COLOR_RESET};

pub fn ansi_256(code: u8) -> String {
    format!("\x1b[38;5;{}m", code)
}

/// Single-color bar for the context window (blue fill, grey empty).
pub fn draw_bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f64).round() as usize;
    let mut bar = String::new();
    for i in 0..width {
        bar.push_str(&ansi_256(if i < filled { COLOR_USAGE } else { COLOR_EMPTY }));
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
    // TODO(human): implement the 4-color dual bar here.
    // Suggested start: convert usage_pct and time_pct to cell counts,
    // then loop over 0..width picking the right color for each cell i.
    todo!()
}
