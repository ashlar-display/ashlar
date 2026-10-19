//! A display connection stack in Rust — HDMI 2.1 first.
//!
//! Ashlar takes a display link from raw capability bytes to a trained link: parse what the
//! sink can do, negotiate the best mode the source, sink and cable can all carry, describe it
//! in InfoFrames, and bring up the link over SCDC with FRL training.
//!
//! Each layer is its own crate and can be used on its own. This crate re-exports them behind
//! one feature per crate, so a single dependency can pull in exactly the layers you need:
//!
//! ```toml
//! [dependencies]
//! ashlar = { version = "0.1", features = ["piaf", "concordance"] }
//! ```
//!
//! | Feature | Crate | Layer |
//! |---|---|---|
//! | `display-types` | [`display_types`] | Shared display capability vocabulary |
//! | `piaf` | [`piaf`] | EDID and DisplayID parsing |
//! | `concordance` | [`concordance`] | HDMI 2.1 mode negotiation |
//! | `cartouche` | [`cartouche`] | InfoFrame encoding and decoding |
//! | `culvert` | [`culvert`] | Typed SCDC register access |
//! | `culvert-async` | [`culvert_async`] | Async companion to culvert |
//! | `plumbob` | [`plumbob`] | FRL link training |
//! | `plumbob-async` | [`plumbob_async`] | Async companion to plumbob |
//! | `hdmi-hal` | [`hdmi_hal`] | `ScdcTransport` and `HdmiPhy` traits |
//! | `hdmi-hal-async` | [`hdmi_hal_async`] | Async companion to hdmi-hal |
//! | `hdmi-hal-i2c-dev` | [`hdmi_hal_i2c_dev`] | Linux userspace backend over `/dev/i2c-N` |
//!
//! `full` enables every crate. `std` (default) and `alloc` are forwarded to the enabled crates
//! that have them; disable default features for `no_std` use.
//!
//! See the [Ashlar website](https://ashlar-display.org) for the full stack.

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "display-types")]
pub use display_types;

#[cfg(feature = "piaf")]
pub use piaf;

#[cfg(feature = "concordance")]
pub use concordance;

#[cfg(feature = "cartouche")]
pub use cartouche;

#[cfg(feature = "culvert")]
pub use culvert;

#[cfg(feature = "culvert-async")]
pub use culvert_async;

#[cfg(feature = "plumbob")]
pub use plumbob;

#[cfg(feature = "plumbob-async")]
pub use plumbob_async;

#[cfg(feature = "hdmi-hal")]
pub use hdmi_hal;

#[cfg(feature = "hdmi-hal-async")]
pub use hdmi_hal_async;

#[cfg(feature = "hdmi-hal-i2c-dev")]
pub use hdmi_hal_i2c_dev;
