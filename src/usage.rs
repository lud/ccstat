use chrono::{DateTime, Duration, Local, Utc};
use std::{fs, io::Write, path::Path};

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub usage_pct: f64,
    pub time_pct: f64,
    pub resets_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct UsageInfo {
    pub five_hour: Option<WindowInfo>,
    pub seven_day: Option<WindowInfo>,
}

fn time_pct(resets_at: DateTime<Utc>, window: Duration) -> f64 {
    let now = Utc::now();
    let start = resets_at - window;
    let total = window.num_seconds() as f64;
    let elapsed = (now - start).num_seconds() as f64;
    (elapsed / total * 100.0).clamp(0.0, 100.0)
}

fn parse_window(v: &serde_json::Value, window: Duration) -> Option<WindowInfo> {
    let usage_pct = v.get("used_percentage")?.as_f64()?;
    let resets_at = v
        .get("resets_at")
        .and_then(|r| r.as_i64())
        .and_then(|s| DateTime::<Utc>::from_timestamp(s, 0));
    let t_pct = resets_at.map(|r| time_pct(r, window)).unwrap_or(0.0);
    Some(WindowInfo {
        usage_pct,
        time_pct: t_pct,
        resets_at,
    })
}

pub fn parse_rate_limits(input: &serde_json::Value) -> UsageInfo {
    let rl = &input["rate_limits"];
    UsageInfo {
        five_hour: parse_window(&rl["five_hour"], Duration::hours(5)),
        seven_day: parse_window(&rl["seven_day"], Duration::days(7)),
    }
}

pub fn log_msg(path: &Path, msg: &str) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{}] {}", Local::now().format("%H:%M:%S"), msg);
    }
}
