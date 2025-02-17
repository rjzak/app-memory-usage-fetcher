// SPDX-License-Identifier: Apache-2.0

fn main() {
    cc::Build::new()
        .file("src/os_function_calls.c")
        .compile("osfuncs");
}
