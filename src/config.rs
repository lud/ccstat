use chrono_tz::Tz;

pub const USAGE_TTL_SECS: u64 = 900;
pub const BAR_WIDTH: usize = 10;
pub const BLOCK: char = '█';

/// ANSI 256-color: consumed usage (blue)
pub const COLOR_USAGE: u8 = 33;
/// ANSI 256-color: elapsed time (red)
pub const COLOR_TIME: u8 = 196;
/// ANSI 256-color: both used and elapsed (purple)
pub const COLOR_OVERLAP: u8 = 93;
/// ANSI 256-color: neither used nor elapsed (grey)
pub const COLOR_EMPTY: u8 = 239;
/// ANSI reset sequence
pub const COLOR_RESET: &str = "\x1b[0m";

pub const DEFAULT_CACHE_PATH: &str = "/tmp/claude-usage-cache.json";
pub const DEFAULT_LOG_PATH: &str = "/tmp/ccstat.log";

pub const DISPLAY_TZ: Tz = chrono_tz::Europe::Paris;
pub const PREFIX_COLOR: u8 = 202;
