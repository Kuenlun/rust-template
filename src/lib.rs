// SPDX-License-Identifier: MIT OR Apache-2.0
// rust-template - Hardened Rust project template
// Copyright (c) 2026 Juan Luis Leal Contreras (Kuenlun)

// Allow `#[coverage(off)]` on test modules under `--cfg coverage_nightly` (nightly-only).
#![cfg_attr(all(test, coverage_nightly), feature(coverage_attribute))]

/// Returns a friendly greeting addressed to `name`.
#[must_use]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn greet_addresses_the_given_name() {
        assert_eq!(greet("world"), "Hello, world!");
    }
}
