use chrono::{DateTime, Duration, Local, Utc};
use serde::Deserialize;
use std::{
    fs,
    io::Write,
    path::Path,
    time::SystemTime,
};
use crate::config::USAGE_TTL_SECS;

#[derive(Debug, Deserialize)]
struct WindowRaw {
    utilization: Option<f64>,
    resets_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct UsageResponse {
    five_hour: Option<WindowRaw>,
    seven_day: Option<WindowRaw>,
}

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

fn parse_response(json: &str) -> Option<UsageInfo> {
    let resp: UsageResponse = serde_json::from_str(json).ok()?;

    let five_hour = resp.five_hour.and_then(|w| {
        let usage_pct = w.utilization?;
        let t_pct = w.resets_at
            .map(|r| time_pct(r, Duration::hours(5)))
            .unwrap_or(0.0);
        Some(WindowInfo { usage_pct, time_pct: t_pct, resets_at: w.resets_at })
    });

    let seven_day = resp.seven_day.and_then(|w| {
        let usage_pct = w.utilization?;
        let t_pct = w.resets_at
            .map(|r| time_pct(r, Duration::days(7)))
            .unwrap_or(0.0);
        Some(WindowInfo { usage_pct, time_pct: t_pct, resets_at: w.resets_at })
    });

    Some(UsageInfo { five_hour, seven_day })
}

fn load_cache(path: &Path, log: &Path) -> Option<UsageInfo> {
    let meta = fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?;
    let age = SystemTime::now()
        .duration_since(modified)
        .unwrap_or_default()
        .as_secs();
    log_msg(log, &format!("Cache age={}s TTL={}s", age, USAGE_TTL_SECS));
    if age >= USAGE_TTL_SECS {
        log_msg(log, "Cache expired");
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    log_msg(log, "Using cached usage");
    parse_response(&content)
}

fn fetch_usage(creds_path: &Path, log: &Path) -> Option<String> {
    let creds = fs::read_to_string(creds_path)
        .map_err(|e| log_msg(log, &format!("Read creds failed: {}", e)))
        .ok()?;
    let creds_val: serde_json::Value = serde_json::from_str(&creds)
        .map_err(|e| log_msg(log, &format!("Parse creds failed: {}", e)))
        .ok()?;
    let token = creds_val["claudeAiOauth"]["accessToken"]
        .as_str()
        .filter(|t| !t.is_empty())
        .map(str::to_string)?;

    log_msg(log, "Fetching usage from API...");
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    match agent
        .get("https://api.anthropic.com/api/oauth/usage")
        .set("Authorization", &format!("Bearer {}", token))
        .set("anthropic-beta", "oauth-2025-04-20")
        .call()
    {
        Ok(resp) => {
            log_msg(log, "HTTP 200");
            resp.into_string().ok()
        }
        Err(ureq::Error::Status(code, resp)) => {
            let body = resp.into_string().unwrap_or_default();
            log_msg(log, &format!("HTTP {}: {:.200}", code, body));
            None
        }
        Err(e) => {
            log_msg(log, &format!("HTTP request failed: {}", e));
            None
        }
    }
}

pub fn load_usage(cache_path: &Path, creds_path: &Path, log: &Path) -> Option<UsageInfo> {
    if let Some(info) = load_cache(cache_path, log) {
        return Some(info);
    }
    let json = fetch_usage(creds_path, log)?;
    let info = parse_response(&json)?;
    if let Err(e) = fs::write(cache_path, &json) {
        log_msg(log, &format!("Write cache failed: {}", e));
    }
    Some(info)
}

pub fn log_msg(path: &Path, msg: &str) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "[{}] {}", Local::now().format("%H:%M:%S"), msg);
    }
}
