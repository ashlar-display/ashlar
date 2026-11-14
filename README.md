# Ashlar

**A display connection stack in Rust — HDMI 2.1 first.**

Ashlar takes a display link from raw capability bytes to a trained link: parse what the
sink can do, negotiate the best mode the source, sink and cable can all carry, describe it
in InfoFrames, and bring up the link over SCDC, with FRL training or over TMDS. Each layer
is its own crate — use one, or use them all.

## Usage

`ashlar` re-exports the stack's crates behind one feature per crate, so a single dependency
pulls in exactly the layers you need:

```toml
[dependencies]
ashlar = { version = "0.3", features = ["piaf", "concordance"] }
```

Each crate is then reachable under its own name, `ashlar::piaf`, `ashlar::concordance` and
so on, in the version range this release of `ashlar` requires (for example any piaf 0.6.x
from 0.6.0).

## Features

| Feature | Crate | Layer |
|---|---|---|
| `display-types` | [display-types](https://crates.io/crates/display-types) | Shared display capability vocabulary |
| `piaf` | [piaf](https://crates.io/crates/piaf) | EDID and DisplayID parsing |
| `concordance` | [concordance](https://crates.io/crates/concordance) | HDMI 2.1 mode negotiation |
| `cartouche` | [cartouche](https://crates.io/crates/cartouche) | InfoFrame encoding and decoding |
| `culvert` | [culvert](https://crates.io/crates/culvert) | Typed SCDC register access |
| `culvert-async` | [culvert-async](https://crates.io/crates/culvert-async) | Async companion to culvert |
| `plumbob` | [plumbob](https://crates.io/crates/plumbob) | FRL link training |
| `plumbob-async` | [plumbob-async](https://crates.io/crates/plumbob-async) | Async companion to plumbob |
| `spiritlevel` | [spiritlevel](https://crates.io/crates/spiritlevel) | TMDS link parameters: clock ratio, scrambling, the sink's `TMDS_Config` |
| `hdmi-hal` | [hdmi-hal](https://crates.io/crates/hdmi-hal) | `ScdcTransport` and `HdmiPhy` traits |
| `hdmi-hal-async` | [hdmi-hal-async](https://crates.io/crates/hdmi-hal-async) | Async companion to hdmi-hal |
| `hdmi-hal-i2c-dev` | [hdmi-hal-i2c-dev](https://crates.io/crates/hdmi-hal-i2c-dev) | Linux userspace backend over `/dev/i2c-N` |

`full` enables every crate. With `culvert` and `plumbob` both enabled, culvert's `Scdc`
implements plumbob's `ScdcClient`, so it drives `FrlTrainer` directly; the same holds for
`culvert-async` and `plumbob-async`. With `display-types` and `spiritlevel` both enabled,
spiritlevel's `SinkTmdsFacts` converts from display-types' `HdmiForumSinkCap`.

## `no_std` builds

`std` (default) and `alloc` are forwarded to the enabled crates that have them; disable
default features for `no_std` use. Every crate except `hdmi-hal-i2c-dev`, a Linux
userspace backend, builds without `std`; CI builds them all together for a bare-metal
target.

## Out of scope

- **Code of its own** — `ashlar` adds nothing to the crates it re-exports; each crate's own
  documentation describes it.
- **Choosing versions per crate** — a release of `ashlar` requires one semver-compatible
  series of each crate (piaf 0.6.x, say). To combine others, depend on the crates
  directly.

## Documentation

- [docs.rs](https://docs.rs/ashlar) — the re-exported crates, with the feature each needs
- [Ashlar website](https://ashlar-display.org) — the full stack, the individual crates, and
  the design principles
- [`doc/setup.md`](doc/setup.md) — build and check commands
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — how to contribute, and how releases reach this crate

## Verifying releases

Releases are built on GitHub Actions and attested with
[SLSA Build Level 2](https://slsa.dev) provenance (from 0.2.0). To verify a release
`.crate` against its signed provenance, install the
[GitHub CLI](https://cli.github.com/) and run:

```sh
gh attestation verify ashlar-X.Y.Z.crate --repo ashlar-display/ashlar
```

The attested `.crate` is attached to each
[GitHub release](https://github.com/ashlar-display/ashlar/releases).

Releases from 0.2.0 are uploaded through crates.io
[trusted publishing](https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html):
the publish workflow obtains a short-lived token via OpenID Connect instead of using a
stored API token, and crates.io rejects uploads made with an API token for this crate.
Each version's publishing details, including the workflow run and commit, are available
from the crates.io API:

```sh
curl -A "your-tool (contact)" https://crates.io/api/v1/crates/ashlar/X.Y.Z
# see .version.trustpub_data
```

0.1.0 was uploaded with an API token; it carries no provenance and has no GitHub release.

## License

Licensed under the [Mozilla Public License 2.0](LICENSE).
