# Gravity Changer (Horizon native mod)

An empty [native mod](https://playhorizon.dev/docs/make-a-mod) for
[Horizon](https://playhorizon.dev), the Minecraft: Java Edition 26.3 client
rewritten in Rust. It is a starting point: a bundle for
[horizon-mixins](https://github.com/horizon-rust/horizon-mixins) that builds
into the game and changes nothing yet.

## The files

| File | What it is |
|---|---|
| `Cargo.toml`, `Cargo.lock` | Runs the mod's unit tests on its own logic: `cargo test` |
| `compatibility.json` | The hook points it needs, and the commits it was made on |
| `mixins/gravity_changer/horizon-mixin.toml` | The manifest: its id, its version, what it hooks |
| `mixins/gravity_changer/runtime.rs` | The mod's own code: becomes the module `crate::game::gravity_changer` |
| `mixins/gravity_changer/list.rs` | Helper: the mod's row in Options > Mods |
| `mixins/gravity_changer/tick.rs` | Helper: called every game tick |

## Tools

You need Rust (stable), Git, Python 3.11+, `rsync`, the
[Horizon launcher](https://playhorizon.dev/install) (0.2.37 or later), and a
clone of the registry's repository:

```sh
git clone https://github.com/horizon-rust/horizon-mods
```

## Build and test

```sh
cargo test                                              # the mod's own logic
cd horizon-mods
python3 native/tools/native.py check /path/to/this/repo # apply on Horizon, cargo check the game
horizon-launcher native-build /path/to/this/repo/mixins/gravity_changer --trust-native-code
```

The first game build downloads Rust and takes 20 to 30 minutes; later ones
compile only what changed. While you work, `horizon-launcher settings
native-profile dev` rebuilds about six times faster. Native mods are not
sandboxed: build only code you trust.

Docs: [Make a mod](https://playhorizon.dev/docs/make-a-mod),
[The bundle](https://playhorizon.dev/docs/mod-manifest),
[Hook points and helpers](https://playhorizon.dev/docs/mod-code).

License: Apache-2.0.
