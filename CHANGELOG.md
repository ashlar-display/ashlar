# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
