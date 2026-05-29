use chrono_tz::Tz;

pub const LONG_BAR_WIDTH: usize = 20;
pub const SHORT_BAR_WIDTH: usize = 10;
pub const BLOCK: char = '█';

// Colors are ANSI palette indices (0-15) so the bar follows the terminal theme
// instead of using fixed RGB values.
/// Theme palette: consumed usage (blue)
pub const COLOR_USAGE: u8 = 4;
/// Theme palette: elapsed time (white)
pub const COLOR_TIME: u8 = 7;
/// Theme palette: usage running ahead of elapsed time (yellow, warning)
pub const COLOR_OVERFLOW: u8 = 3;
/// Theme palette: neither used nor elapsed (grey)
pub const COLOR_EMPTY: u8 = 8;
/// ANSI reset sequence
pub const COLOR_RESET: &str = "\x1b[0m";

pub const DEFAULT_DIR: &str = "/tmp/claude-statusline";

pub const DISPLAY_TZ: Tz = chrono_tz::Europe::Paris;
pub const PREFIX_COLOR: u8 = COLOR_OVERFLOW;
