# Contributing to ashlar

Thanks for your interest in contributing. This document covers the basics.

## Getting started

See [`doc/setup.md`](doc/setup.md) for build and check commands.

`ashlar` only re-exports the stack's crates. A change to what one of them does belongs in
that crate's repository; the [Ashlar website](https://ashlar-display.org/crates.html)
lists them.

## Issues and pull requests

**Open an issue first** if you're unsure whether something belongs here or in one of the
re-exported crates, or if you want to discuss a change before implementing it. For small,
self-contained fixes a PR on its own is fine.

- Bug reports: what you enabled (the features), what you expected, and what happened.
- Feature requests: a brief description of what you need and why is enough to start a
  conversation.
- PRs: keep them focused. One logical change per PR makes review faster and keeps
  history readable.

## Coding standards

- Run `cargo fmt` and `cargo clippy --all-targets --all-features -- -D warnings` before
  pushing ([doc/setup.md](doc/setup.md) lists the commands CI runs).
- Public items need rustdoc comments, and intra-doc links must resolve
  (`cargo rustdoc --all-features -- -D warnings` must pass).
- `#![forbid(unsafe_code)]` is enforced; no unsafe code.
- Each re-exported crate stays behind its own feature, every feature builds on its own,
  and `std` and `alloc` are forwarded to the crates that have them, so the features stay
  additive.
- A claim about HDMI behaviour (a rule, a default, a threshold, a register encoding) cites
  at least two independent public implementations (repository, commit, file and line).
  Where implementations differ, the behaviour is a setting with the majority as default.

## Commit and PR expectations

- Write commit messages in the imperative mood ("Add support for …", not "Added …").
- Keep commits logically atomic. A PR that touches three unrelated things should be three
  commits (or three PRs).
- CI must be green before a PR can merge: fmt, clippy with every feature and with none,
  docs, the tests, each feature on its own, the bare-metal `no_std` build, and the same
  tests and build on the MSRV (Rust 1.85).

## Publishing and upstream dependencies

`ashlar` re-exports every crate of the stack, so their versions are part of its API:
moving to a breaking release of one of them is a breaking release of `ashlar`, unless the
version requirement keeps accepting the old release as well. A crate's new release reaches
`ashlar` after it is published to crates.io:

1. The crate is released and published from its own repository.
2. Update its version requirement in `Cargo.toml` here, and the crate's features if they
   changed.
3. Merge and tag a release in this repo (`vX.Y.Z`); the publish CI workflow uploads it.

## Review process

PRs are reviewed on a best-effort basis. Expect feedback within a few days; if you
haven't heard back in a week feel free to ping the thread. Reviews aim to be
constructive — if something needs to change, the reviewer will explain why. Approval
from the maintainer is required to merge.

## Code of Conduct

This project follows the [Contributor Covenant 3.0](https://github.com/ashlar-display/.github/blob/HEAD/CODE_OF_CONDUCT.md),
the code of conduct of every [Ashlar](https://ashlar-display.org) repository. Please read it
before participating.
