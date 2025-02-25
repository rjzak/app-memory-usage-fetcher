[![Build](https://github.com/rjzak/app-memory-usage-fetcher/actions/workflows/build.yml/badge.svg)](https://github.com/rjzak/app-memory-usage-fetcher/actions/workflows/build.yml)
[![Lint](https://github.com/rjzak/app-memory-usage-fetcher/actions/workflows/lint.yml/badge.svg)](https://github.com/rjzak/app-memory-usage-fetcher/actions/workflows/lint.yml)
[![Crates.io Version](https://img.shields.io/crates/v/app-memory-usage-fetcher)](https://crates.io/crates/app-memory-usage-fetcher)

## Application Memory Usage Fetcher

A super simple crate which queries the supported operating system for the number of bytes used. Returns `None` for unsupported operating systems.

This crate was created after trying to find this functionality from an existing crate, and instead I found [instances](https://www.reddit.com/r/rust/comments/b9irmd/question_how_to_determine_memory_usage_of_current/) [of](https://stackoverflow.com/questions/74558630/is-there-a-simple-way-to-measure-total-memory-consumption) [people](https://users.rust-lang.org/t/tracking-memory-usage/98451) [asking](https://www.reddit.com/r/rust/comments/twqi7e/how_to_investigate_memory_usage_of_your_rust/) for it. So I decided to make it.
