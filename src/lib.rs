// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../readme.md")]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![allow(clippy::doc_markdown)] // Clippy has issues with some names in the OS list
#![deny(missing_docs)]

use std::num::NonZeroU64;

/// Library version
pub const APP_MEMORY_USAGE_FETCHER_VERSION: &str = env!("CARGO_PKG_VERSION");

extern "C" {
    fn getMemoryUsage() -> i64;
}

const KILOBYTE: f64 = 1024.0f64;
const MEGABYTE: f64 = 1024.0f64 * 1024.0f64;
const GIGABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64;
const TERABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64 * 1024.0f64;

/// Application memory usage in bytes
/// ```
///  assert!(app_memory_usage_fetcher::get_memory_usage_bytes().is_some());
/// ```
#[inline]
#[must_use]
pub fn get_memory_usage_bytes() -> Option<NonZeroU64> {
    let bytes = unsafe { getMemoryUsage() };
    if bytes <= 0 {
        None
    } else {
        NonZeroU64::new(bytes.cast_unsigned())
    }
}

/// Application memory usage in kilobytes (KiB)
#[inline]
#[must_use]
pub fn get_memory_usage_kbytes() -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    get_memory_usage_bytes().map(|m| m.get() as f64 / KILOBYTE)
}

/// Application memory usage in megabytes (MiB)
#[inline]
#[must_use]
pub fn get_memory_usage_mbytes() -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    get_memory_usage_bytes().map(|m| m.get() as f64 / MEGABYTE)
}

/// Application memory usage in gigabytes (GiB)
#[inline]
#[must_use]
pub fn get_memory_usage_gbytes() -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    get_memory_usage_bytes().map(|m| m.get() as f64 / GIGABYTE)
}

/// Application memory usage in terabytes (TiB)
#[inline]
#[must_use]
pub fn get_memory_usage_tbytes() -> Option<f64> {
    #[allow(clippy::cast_precision_loss)]
    get_memory_usage_bytes().map(|m| m.get() as f64 / TERABYTE)
}

/// Application memory usage with unit
///
/// ```
/// if let Some(usage) = app_memory_usage_fetcher::get_memory_usage_string() {
///     println!("Memory usage: {}", usage);
/// }
/// ```
#[inline]
#[must_use]
pub fn get_memory_usage_string() -> Option<String> {
    if let Some(bytes) = get_memory_usage_bytes() {
        #[allow(clippy::cast_precision_loss)]
        let bytes = bytes.get() as f64;
        match bytes {
            0.0..=KILOBYTE => Some(format!("{bytes:.2} bytes")),
            KILOBYTE..=MEGABYTE => Some(format!("{:.2} KB", bytes / KILOBYTE)),
            MEGABYTE..=GIGABYTE => Some(format!("{:.2} MB", bytes / MEGABYTE)),
            GIGABYTE..=TERABYTE => Some(format!("{:.2} GB", bytes / GIGABYTE)),
            _ => Some(format!("{:.2} TB", bytes / TERABYTE)),
        }
    } else {
        None
    }
}
