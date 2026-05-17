# rust-template

[![CI](https://github.com/Kuenlun/rust-template/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Kuenlun/rust-template/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Kuenlun/rust-template/branch/master/graph/badge.svg)](https://codecov.io/gh/Kuenlun/rust-template)
<!-- [![Crates.io](https://img.shields.io/crates/v/rust-template.svg)](https://crates.io/crates/rust-template) -->
[![Docs.rs](https://docs.rs/rust-template/badge.svg)](https://docs.rs/rust-template)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Hardened Rust project template: strict linting, 100% coverage enforcement, dual MIT/Apache-2.0 licensing, automated releases with cross-compiled binaries.

## What's included

- **Toolchain** (`rust-toolchain.toml`): pinned nightly with rustfmt, clippy, llvm-tools-preview.
- **Linting**: `clippy::pedantic` and `clippy::nursery` denied, `unwrap_used` / `expect_used` denied (allowed in tests via `clippy.toml`), `unsafe_code = forbid`, `warnings = deny`.
- **Coverage**: lockpick enforces 100% functions, lines, regions, and branches, and checks a byte-equality license header on every source file.
- **Release profile**: fat LTO, single codegen unit, `panic = "abort"`, debuginfo stripped. Local builds also use `target-cpu=native` on x86_64 / aarch64 (`.cargo/config.toml`).
- **CI** (`.github/workflows/rust.yml`): runs `lockpick -v` on Linux, Windows, macOS; uploads LCOV to Codecov; gate job that fails if any matrix job fails.
- **Releases** (`.github/workflows/release-plz.yml`): release-plz opens release PRs and tags on merge to `master`. Each release gets six prebuilt binaries attached: Linux musl, Windows MSVC, and macOS, x86_64 and aarch64. A `workflow_dispatch` dry-run builds the matrix without publishing.
- **PR titles** (`.github/workflows/pr-title-linter.yml`): Conventional Commits enforced via `amannn/action-semantic-pull-request`.
- **Pre-commit hook** (`.githooks/pre-commit`): runs `lockpick` before every commit.

## Setup checklist

After cloning or forking, walk through these before the first commit.

### Crate identity

In `Cargo.toml`:

- [ ] `name`: rename to your crate name.
- [ ] `authors`: update to your name and email.
- [ ] `description`, `repository`, `keywords`, `categories`: uncomment and fill in.
- [ ] `publish = false`: remove when ready to publish to crates.io.

In `README.md`: update the title, badge URLs (CI, Codecov, Docs.rs), and uncomment the Crates.io badge once the crate is published.

In `.github/workflows/release-plz.yml`, the `upload-assets` job hardcodes `bin: rust-template`. Rename it to match the new crate name, or delete the entire `upload-assets` job if your project is library-only.

### License header

Update `.github/license_header.rs` (project name and copyright line) so it matches every source file. Keep `LICENSE-MIT` and `LICENSE-APACHE`, or replace them and update `license` in `Cargo.toml` accordingly.

### Toolchain pin

The nightly channel is referenced in three places and must stay in sync:

- `rust-toolchain.toml`
- `.github/workflows/rust.yml`
- `.github/workflows/release-plz.yml`

Bump them together, or switch to `channel = "nightly"` for a rolling pin.

### Git hooks

Enable the pre-commit hook on a fresh clone:

```sh
git config core.hooksPath .githooks
```

### GitHub secrets

In repo Settings > Secrets and variables > Actions:

| Secret | Purpose | Source |
|---|---|---|
| `RELEASE_PLZ_TOKEN` | Lets release-plz push commits and open PRs | GitHub PAT with `repo` scope, or fine-grained with `contents: write` and `pull-requests: write` |
| `CODECOV_TOKEN` | Uploads coverage reports | Enable the repo on [codecov.io](https://codecov.io) and copy the upload token |

To publish to crates.io, also add `CARGO_REGISTRY_TOKEN` and uncomment the matching line in `release-plz.yml`.

## Usage

```sh
# Run every local check (format, clippy, coverage, license headers, unused deps, audit)
lockpick -v

# Coverage report (HTML, opens browser), excluding test modules
cargo cov

# Coverage including test modules
cargo cov-tests
```

The `cov-all` and `cov-tests-all` variants also run `#[ignore]`d tests. See `.cargo/config.toml`.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option. Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion shall be dual-licensed as above, without any additional terms or conditions.
