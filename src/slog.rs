//! Append-only file log on the SD card, so a boot crash leaves a trace of how far startup got.

use std::{fs::OpenOptions, io::Write};

const PATH: &str = "sd:/ultimate/smashline-compat.log";

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(PATH) {
        let _ = writeln!(f, "{}", msg);
        let _ = f.sync_all();
    }
}

static SEEN: std::sync::Mutex<Vec<(&'static str, u64)>> = std::sync::Mutex::new(Vec::new());

/// Log `tag value` only the first time this pair is seen.
pub fn once(tag: &'static str, value: u64) {
    let Ok(mut seen) = SEEN.lock() else { return };
    if seen.contains(&(tag, value)) {
        return;
    }
    seen.push((tag, value));
    drop(seen);
    log(&format!("{} {:#x}", tag, value));
}

#[macro_export]
macro_rules! install_logged {
    ($($h:ident),* $(,)?) => { $( $crate::slog!("hook {}", stringify!($h)); skyline::install_hook!($h); )* };
}

#[macro_export]
macro_rules! slog {
    ($($arg:tt)*) => { $crate::slog::log(&format!($($arg)*)) };
}
