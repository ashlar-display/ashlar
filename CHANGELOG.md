# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`spiritlevel` feature** — re-exports [spiritlevel](https://github.com/ashlar-display/spiritlevel)
  0.1, which chooses a TMDS link: the bit clock ratio, scrambling and the sink's
  `TMDS_Config`. With `display-types` enabled too, its `SinkTmdsFacts` converts from
  `HdmiForumSinkCap`. Part of `full`, and of the `no_std` build in CI.
- **docs.rs marks each re-exported crate with the feature that enables it** (the `doc_cfg`
  badge, enabled only in docs.rs builds).

### Changed

- **crates.io lists `ashlar` under `no-std::no-alloc`** as well: with default features off
  it builds without an allocator, as its re-exported crates do.

### Internal

- **The published `.crate` holds only what the crate needs**: `include` in `Cargo.toml`
  limits it to the sources, `doc/`, the README, changelog and license; it also shipped the
  CI workflows, Dependabot configuration and `.gitignore`.
- **`#![deny(missing_docs)]` in `lib.rs`**: an undocumented public item fails a local
  build, not only CI's rustdoc step.
- **The documentation is built as docs.rs builds it, weekly**: `docs.yml` runs
  `cargo doc --all-features` with nightly and `--cfg docsrs` (and `-D warnings`), so a
  nightly change to the `doc_cfg` feature badges shows up before it breaks a docs.rs build.
- **The README follows the stack's crate layout**: Usage, a table of the features and the
  crates they enable, `no_std` builds, Out of scope, Documentation, Verifying releases
  (with this crate's release history) and License. Its Rust examples, if it gains any, run
  as doctests (`ReadmeDoctests` in `lib.rs`).
- **CI runs with a read-only token**: `ci.yml` sets `permissions: contents: read`; its jobs
  ran with the repository's default token permissions.
- **CI's test job does not keep the checkout's git credentials** (`persist-credentials:
  false`); it never pushes.
- **CI fails on a `[patch]` section in `Cargo.toml`**, on `main` and in the publish
  workflow's `verify` job: `cargo package` drops it, so a release built with one would not
  build from crates.io.
- **Clippy lints all targets, with every feature and with none**, in CI and in the
  publish workflow's `verify` job; it linted only the library, with every feature.
- **rustdoc runs with `-D warnings`**, as docs.rs builds, in CI and in the publish
  workflow's `verify` job: `-D missing_docs` alone let broken intra-doc links through.
- **The MSRV is checked**: an `MSRV` job in CI and in the publish workflow (which waits for
  it) runs the tests with every feature and the `no_std` build of every crate that has one
  on the `rust-version` from `Cargo.toml` (1.85) with `cargo msrv verify`; the crate
  declared it, and nothing checked it.
- **`CONTRIBUTING.md` and `doc/setup.md`**: how to contribute (and where a change to a
  re-exported crate belongs), the checks CI runs, building each feature on its own, the
  `no_std` build, the MSRV check, and how a crate's release reaches `ashlar`.
- **The README and `doc/setup.md` say what is required and tested**: a release requires a
  semver-compatible version range of each crate, not one version, and `cargo test` compiles
  every feature (there are no tests of its own yet).

## [0.3.0] - 2026-11-03

### Breaking changes

- **display-types 0.5 across the stack.** The re-exported crates move to releases that
  agree on one display-types, so values pass from one layer to the next (piaf →
  concordance → cartouche → culvert → plumbob); with display-types 0.4.3 for some layers
  and 0.5 for others, rustc saw two `HdmiForumFrl` types. Across a breaking version:
  display-types 0.5 (DTD sources name their block, `VideoMode::h_blank`, the preferred
  timing), piaf 0.6 (new warnings and decode options; see its 0.5.0 and 0.6.0 entries) and
  concordance 0.3 (the FRL capacity check in the default rules). hdmi-hal-i2c-dev goes to
  0.4.0 with no API change. The rest are patch releases that accept display-types 0.5:
  cartouche 0.3.1, culvert 0.3.1, culvert-async 0.3.1, plumbob 0.4.1, plumbob-async 0.3.1,
  hdmi-hal 0.6.1 and hdmi-hal-async 0.3.1. Each crate's changelog has the details and the
  migration. Code that names these crates both directly and through `ashlar` moves them
  together.

### Fixed

- **The `piaf` feature builds without `std`.** piaf 0.4.2 took display-types and
  thiserror with their default features, so `ashlar` with `piaf` and no default features
  failed on a `no_std` target; piaf 0.5 fixed that.

- **`culvert` and `plumbob` together now connect.** Enabling both features turns on
  culvert's `plumbob` feature, so culvert's `Scdc` implements plumbob's `ScdcClient` and
  can be handed to `FrlTrainer`; before, it could not without naming culvert directly.
  `culvert-async` and `plumbob-async` likewise.

### Internal

- **CI builds `piaf` for `thumbv7em-none-eabi`** with the other `no_std` features, in
  `ci.yml` and in the verify job of `publish.yml`; it was left out while piaf needed `std`.

## [0.2.0] - 2026-10-25

### Breaking changes

- **The re-exported crates move to their October 2026 releases**, most across a breaking
  version: hdmi-hal 0.6, hdmi-hal-async 0.3, hdmi-hal-i2c-dev 0.3, plumbob 0.4,
  plumbob-async 0.3, culvert 0.3 and culvert-async 0.3, with display-types 0.4.3
  (additive). They carry the SCDC block reads and the CED checksum: `read_block` is a
  required method, `read_ced` takes the lane count and returns a checked reading, and
  plumbob's `ScdcClient` no longer reads CED; each crate's changelog has the details and
  the migration. piaf 0.4.2, concordance 0.2.0 and cartouche 0.3.0 are unchanged. Code
  that names these crates both directly and through `ashlar` moves them together.

### Added

- CI, security audit, Dependabot and the release workflows (release prep, release tag,
  publish with SLSA provenance and crates.io trusted publishing), matching the other
  crates in the stack.

### Changed

- **Points to the Ashlar website.** `homepage`, the README and the crate documentation link
  to [ashlar-display.org](https://ashlar-display.org) instead of the GitHub organisation.
  Keywords are aligned with the rest of the stack: `ashlar` is added, `frl` is dropped.
- The README's and the crate documentation's dependency example asks for `ashlar` 0.2.

## [0.1.0] - 2026-10-19

### Added

- Initial release: the `ashlar` facade crate. One optional feature per stack crate
  (`display-types`, `piaf`, `concordance`, `cartouche`, `culvert`, `culvert-async`,
  `plumbob`, `plumbob-async`, `hdmi-hal`, `hdmi-hal-async`, `hdmi-hal-i2c-dev`), each
  re-exported under its own name; `full` enables all of them. `std` (default) and `alloc`
  are forwarded to the enabled crates that have them.
