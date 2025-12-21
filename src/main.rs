// SPDX-License-Identifier: Apache-2.0

use std::process::ExitCode;

fn main() -> ExitCode {
    if app_memory_usage_fetcher::get_memory_usage_bytes().is_none() {
        eprintln!("Operating system {} not supported.", std::env::consts::OS);
        ExitCode::FAILURE
    } else {
        println!(
            "Memory usage: {} bytes.",
            app_memory_usage_fetcher::get_memory_usage_bytes().unwrap()
        );
        println!(
            "Memory usage: {:.2} kilobytes.",
            app_memory_usage_fetcher::get_memory_usage_kbytes().unwrap()
        );
        println!(
            "Memory usage: {:.2} megabytes.",
            app_memory_usage_fetcher::get_memory_usage_mbytes().unwrap()
        );
        println!(
            "Memory usage: {:.2} gigabytes.",
            app_memory_usage_fetcher::get_memory_usage_gbytes().unwrap()
        );
        println!(
            "Memory usage str: {}",
            app_memory_usage_fetcher::get_memory_usage_string().unwrap()
        );
        ExitCode::SUCCESS
    }
}
