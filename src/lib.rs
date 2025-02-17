// SPDX-License-Identifier: Apache-2.0

#![doc = include_str!("../readme.md")]
#![deny(missing_docs)]

/// Library version
pub const APP_MEMORY_USAGE_FETCHER_VERSION: &str = env!("CARGO_PKG_VERSION");

extern "C" {
    fn getMemoryUsage() -> i64;
}

/// Application memory usage in bytes
/// ```
///  assert!(app_memory_usage_fetcher::get_memory_usage_bytes() > 0);
/// ```
#[inline]
pub fn get_memory_usage_bytes() -> i64 {
    unsafe { getMemoryUsage() }
}

/// Application memory usage in kilobytes
#[inline]
pub fn get_memory_usage_kbytes() -> f64 {
    unsafe { getMemoryUsage() as f64 / 1024.0f64 }
}

/// Application memory usage in megabytes
#[inline]
pub fn get_memory_usage_mbytes() -> f64 {
    const MEGABYTE: f64 = 1024.0f64 * 1024.0f64;
    unsafe { getMemoryUsage() as f64 / MEGABYTE }
}

/// Application memory usage in gigabytes
#[inline]
pub fn get_memory_usage_gbytes() -> f64 {
    const GIGABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64;
    unsafe { getMemoryUsage() as f64 / GIGABYTE }
}

/// Application memory usage in terabytes
#[inline]
pub fn get_memory_usage_tbytes() -> f64 {
    const TERABYTE: f64 = 1024.0f64 * 1024.0f64 * 1024.0f64 * 1024.0f64;
    unsafe { getMemoryUsage() as f64 / TERABYTE }
}
