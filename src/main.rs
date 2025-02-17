// SPDX-License-Identifier: Apache-2.0

use std::process::ExitCode;

fn main() -> Result<ExitCode, ExitCode> {
    if app_memory_usage_fetcher::get_memory_usage_bytes() < 0 {
        eprintln!("Operating system {} not supported.", std::env::consts::OS);
        Err(ExitCode::FAILURE)
    } else {
        println!(
            "Memory usage: {} bytes.",
            app_memory_usage_fetcher::get_memory_usage_bytes()
        );
        println!(
            "Memory usage: {:.2} kilobytes.",
            app_memory_usage_fetcher::get_memory_usage_kbytes()
        );
        println!(
            "Memory usage: {:.2} megabytes.",
            app_memory_usage_fetcher::get_memory_usage_mbytes()
        );
        println!(
            "Memory usage: {:.2} gigabytes.",
            app_memory_usage_fetcher::get_memory_usage_gbytes()
        );
        Ok(ExitCode::SUCCESS)
    }
}
