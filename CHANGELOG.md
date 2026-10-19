# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CI, security audit, Dependabot and the release workflows (release prep, release tag,
  publish with SLSA provenance and crates.io trusted publishing), matching the other
  crates in the stack.

## [0.1.0] - 2026-10-19

### Added

- Initial release: the `ashlar` facade crate. One optional feature per stack crate
  (`display-types`, `piaf`, `concordance`, `cartouche`, `culvert`, `culvert-async`,
  `plumbob`, `plumbob-async`, `hdmi-hal`, `hdmi-hal-async`, `hdmi-hal-i2c-dev`), each
  re-exported under its own name; `full` enables all of them. `std` (default) and `alloc`
  are forwarded to the enabled crates that have them.
