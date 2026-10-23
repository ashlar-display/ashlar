# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- CI, security audit, Dependabot and the release workflows (release prep, release tag,
  publish with SLSA provenance and crates.io trusted publishing), matching the other
  crates in the stack.

### Changed

- **Points to the Ashlar website.** `homepage`, the README and the crate documentation link
  to [ashlar-display.org](https://ashlar-display.org) instead of the GitHub organisation.
  Keywords are aligned with the rest of the stack: `ashlar` is added, `frl` is dropped.
- **Inherit license file.** The license file is removed and inherited from https://github.com/ashlar-display/.github

## [0.1.0] - 2026-10-19

### Added

- Initial release: the `ashlar` facade crate. One optional feature per stack crate
  (`display-types`, `piaf`, `concordance`, `cartouche`, `culvert`, `culvert-async`,
  `plumbob`, `plumbob-async`, `hdmi-hal`, `hdmi-hal-async`, `hdmi-hal-i2c-dev`), each
  re-exported under its own name; `full` enables all of them. `std` (default) and `alloc`
  are forwarded to the enabled crates that have them.
