// SPDX-License-Identifier: Apache-2.0

fn main() {
    println!("cargo:rerun-if-changed=src/os_function_calls.c");
    cc::Build::new()
        .file("src/os_function_calls.c")
        .compile("osfuncs");
}
