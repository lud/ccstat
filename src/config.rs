use chrono_tz::Tz;

pub const USAGE_TTL_SECS: u64 = 900;
pub const LONG_BAR_WIDTH: usize = 20;
pub const SHORT_BAR_WIDTH: usize = 10;
pub const BLOCK: char = '█';

/// ANSI 256-color: consumed usage (blue)
pub const COLOR_USAGE: u8 = 33;
/// ANSI 256-color: elapsed time (red)
pub const COLOR_TIME: u8 = 137;
/// ANSI 256-color: both used and elapsed (purple)
pub const COLOR_OVERLAP: u8 = 93;
/// ANSI 256-color: neither used nor elapsed (grey)
pub const COLOR_EMPTY: u8 = 239;
/// ANSI reset sequence
pub const COLOR_RESET: &str = "\x1b[0m";

pub const DEFAULT_DIR: &str = "/tmp/claude-statusline";

pub const DISPLAY_TZ: Tz = chrono_tz::Europe::Paris;
pub const PREFIX_COLOR: u8 = 202;
