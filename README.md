# Ashlar

**A display connection stack in Rust — HDMI 2.1 first.**

Ashlar takes a display link from raw capability bytes to a trained link: parse what the
sink can do, negotiate the best mode the source, sink and cable can all carry, describe it
in InfoFrames, and bring up the link over SCDC with FRL training. Each layer is its own
crate — use one, or use them all.

## This crate

`ashlar` re-exports the stack's crates behind one feature per crate, so a single dependency
pulls in exactly the layers you need:

```toml
[dependencies]
ashlar = { version = "0.1", features = ["piaf", "concordance"] }
```

`full` enables every crate. `std` (default) and `alloc` are forwarded to the enabled crates
that have them; disable default features for `no_std` use.

See the [Ashlar website](https://ashlar-display.org) for the full stack, the individual
crates, and the design principles.

Licensed under MPL-2.0.
