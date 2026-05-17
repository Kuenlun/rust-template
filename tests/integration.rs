// SPDX-License-Identifier: MIT OR Apache-2.0
// rust-template - Hardened Rust project template
// Copyright (c) 2026 Juan Luis Leal Contreras (Kuenlun)

use assert_cmd::Command;

#[test]
fn main_prints_greeting() {
    Command::cargo_bin("rust-template")
        .expect("binary must be findable")
        .assert()
        .success()
        .stdout("Hello, world!\n");
}
