mod bar;
mod config;
mod usage;

use chrono::{DateTime, Utc};
use config::*;
use std::{
    io::{self, Read},
    path::PathBuf,
    process::Command,
};
use usage::{WindowInfo, load_usage, log_msg};

struct Args {
    prefix: Option<String>,
    cache: PathBuf,
    log: PathBuf,
    creds: PathBuf,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let home = std::env::var("HOME").unwrap_or_default();

    let mut prefix = None;
    let mut cache = PathBuf::from(DEFAULT_CACHE_PATH);
    let mut log = PathBuf::from(DEFAULT_LOG_PATH);
    let mut creds = PathBuf::from(format!("{}/.claude/.credentials.json", home));

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--prefix" if i + 1 < raw.len() => {
                prefix = Some(raw[i + 1].clone());
                i += 2;
            }
            "--cache" if i + 1 < raw.len() => {
                cache = expand_home(&raw[i + 1]);
                i += 2;
            }
            "--log" if i + 1 < raw.len() => {
                log = expand_home(&raw[i + 1]);
                i += 2;
            }
            "--creds" if i + 1 < raw.len() => {
                creds = expand_home(&raw[i + 1]);
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }
    Args {
        prefix,
        cache,
        log,
        creds,
    }
}

fn expand_home(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(format!("{}/{}", home, rest))
    } else {
        PathBuf::from(s)
    }
}

fn git_branch(cwd: &str) -> String {
    Command::new("git")
        .args(["-C", cwd, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn shorten_cwd(cwd: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    if !home.is_empty() && cwd.starts_with(&home) {
        format!("~{}", &cwd[home.len()..])
    } else {
        cwd.to_string()
    }
}

fn fmt_five_h_reset(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&DISPLAY_TZ).format("%H:%M").to_string()
}

fn fmt_seven_d_reset(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&DISPLAY_TZ).format("%a %-Hh").to_string()
}

fn render_window(
    label: &str,
    info: &WindowInfo,
    fmt_reset: impl Fn(DateTime<Utc>) -> String,
) -> String {
    let b = bar::draw_dual_bar(info.usage_pct, info.time_pct, BAR_WIDTH);
    let pct = info.usage_pct.round() as u32;
    let reset = info.resets_at.map(fmt_reset).unwrap_or_else(|| "--".into());
    format!("{} {} {}% {}", label, b, pct, reset)
}

fn main() {
    let args = parse_args();

    let mut stdin_buf = String::new();
    io::stdin().read_to_string(&mut stdin_buf).unwrap_or(0);
    let input: serde_json::Value = serde_json::from_str(&stdin_buf).unwrap_or_default();

    log_msg(&args.log, "ccstat starting");

    let usage = load_usage(&args.cache, &args.creds, &args.log);

    let five_h_display = usage
        .as_ref()
        .and_then(|u| u.five_hour.as_ref())
        .map(|w| render_window("5h", w, fmt_five_h_reset))
        .unwrap_or_else(|| format!("5h {} --% --", bar::draw_bar(0.0, BAR_WIDTH)));

    let seven_d_display = usage
        .as_ref()
        .and_then(|u| u.seven_day.as_ref())
        .map(|w| render_window("7d", w, fmt_seven_d_reset))
        .unwrap_or_else(|| format!("7d {} --% --", bar::draw_bar(0.0, BAR_WIDTH)));

    let ctx_raw = &input["context_window"]["used_percentage"];
    let ctx_pct = ctx_raw.as_f64().unwrap_or(0.0);
    let ctx_bar = bar::draw_bar(ctx_pct, BAR_WIDTH);
    let ctx_display = if ctx_raw.is_null() {
        "n/a".to_owned()
    } else {
        format!("{}%", ctx_pct.round() as u32)
    };

    let cwd = input["cwd"]
        .as_str()
        .or_else(|| input["workspace"]["current_dir"].as_str())
        .unwrap_or("");
    let cwd_short = shorten_cwd(cwd);
    let branch = if cwd.is_empty() {
        String::new()
    } else {
        git_branch(cwd)
    };
    let location = if branch.is_empty() {
        cwd_short
    } else {
        format!("{}  {}", cwd_short, branch)
    };

    let prefix_part = args
        .prefix
        .map(|p| {
            format!(
                "{}({}){} -- ",
                bar::ansi_color_256(PREFIX_COLOR),
                p,
                COLOR_RESET
            )
        })
        .unwrap_or_default();

    println!(
        "{}{} | {} | Ctx {} {} | {}",
        prefix_part, five_h_display, seven_d_display, ctx_bar, ctx_display, location
    );
}
