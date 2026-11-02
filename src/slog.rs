//! Append-only file log on the SD card, so a boot crash leaves a trace of how far startup got.

use std::{fs::OpenOptions, io::Write};

const PATH: &str = "sd:/ultimate/smashline-compat.log";

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(PATH) {
        let _ = writeln!(f, "{}", msg);
        let _ = f.sync_all();
    }
}

#[macro_export]
macro_rules! slog {
    ($($arg:tt)*) => { $crate::slog::log(&format!($($arg)*)) };
}
