// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../readme.md")]
#![deny(missing_docs)]

use std::num::NonZeroU64;

/// Library version
pub const APP_MEMORY_USAGE_FETCHER_VERSION: &str = env!("CARGO_PKG_VERSION");

extern "C" {
    fn getMemoryUsage() -> i64;
}

/// Application memory usage in bytes
/// ```
///  assert!(app_memory_usage_fetcher::get_memory_usage_bytes().is_some());
/// ```
#[inline]
pub fn get_memory_usage_bytes() -> Option<NonZeroU64> {
    let bytes = unsafe { getMemoryUsage() };
    if bytes <= 0 {
        None
    } else {
        NonZeroU64::new(bytes as u64)
    }
}

/// Application memory usage in kilobytes (KiB)
#[inline]
pub fn get_memory_usage_kbytes() -> Option<f64> {
    get_memory_usage_bytes().map(|m| m.get() as f64 / 1024.0f64)
}

/// Application memory usage in megabytes (MiB)
#[inline]
pub fn get_memory_usage_mbytes() -> Option<f64> {
    const MEGABYTE: f64 = 1024.0f64 * 1024.0f64;
    get_memory_usage_bytes().map(|m| m.get() as f64 / MEGABYTE)
}

/// Application memory usage in gigabytes (GiB)
#[inline]
pub fn get_memory_usage_gbytes() -> Option<f64> {
    const GIGABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64;
    get_memory_usage_bytes().map(|m| m.get() as f64 / GIGABYTE)
}

/// Application memory usage in terabytes (TiB)
#[inline]
pub fn get_memory_usage_tbytes() -> Option<f64> {
    const TERABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64 * 1024.0f64;
    get_memory_usage_bytes().map(|m| m.get() as f64 / TERABYTE)
}
