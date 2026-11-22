# Development Setup

**Requirements:** Rust 1.85+ (stable). Install via [rustup](https://rustup.rs/).

## Clone and build

```sh
git clone https://github.com/ashlar-display/ashlar.git
cd ashlar
cargo build --all-features
```

## Running checks

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo rustdoc --all-features -- -D warnings
cargo test --all-features
```

`ashlar` has no code of its own beyond the re-exports and no tests yet: `cargo test` compiles
every feature, and would run any Rust example added to the README (`ReadmeDoctests`).

## Each feature on its own

Every re-exported crate is an optional feature, and each must build on its own. CI builds
them one at a time:

```sh
for FEATURE in $(cargo metadata --no-deps --format-version 1 \
    | jq -r '.packages[0].features | keys[] | select(. != "default" and . != "std" and . != "alloc" and . != "full")'); do
  cargo build --locked --features "$FEATURE"
done
```

## `no_std` builds

Every crate except `hdmi-hal-i2c-dev` builds without `std`. CI builds them together on a
bare target, which has no `std` for a dependency to pull in:

```sh
rustup target add thumbv7em-none-eabi
cargo build --no-default-features --target thumbv7em-none-eabi \
  --features display-types,piaf,concordance,cartouche,culvert,culvert-async,plumbob,plumbob-async,spiritlevel,hdmi-hal,hdmi-hal-async
```

## Checking the MSRV

The MSRV is `rust-version` in `Cargo.toml`. CI checks it with
[`cargo-msrv`](https://github.com/foresterre/cargo-msrv), which runs the checks on that
toolchain:

```sh
cargo install cargo-msrv --version 0.19.6 --locked
cargo msrv verify -- cargo test --locked --all-features
cargo msrv verify --target thumbv7em-none-eabi \
  -- cargo build --locked --no-default-features --target thumbv7em-none-eabi \
  --features display-types,piaf,concordance,cartouche,culvert,culvert-async,plumbob,plumbob-async,spiritlevel,hdmi-hal,hdmi-hal-async
```
