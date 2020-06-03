# Rust + Qt + GStreamer example

Simple video player made on top of GStreamer + Qt + Rust with the help of [gstreamer-rs]([gstreamer-rs](https://github.com/sdroege/gstreamer-rs)), [rust-qt-bindings-generator](https://github.com/KDE/rust-qt-binding-generator). I referrerd to [QtQuickPlayer](https://github.com/ivasilev/QtQuickPlayer) and [Quemail](https://gitlab.com/rhn/quemail) to figure out how they manage to integrate Qt+GStreamer and Qt+Rust, respectively.

This is my learning project. Feel free to make PRs and issues if you have better ideas of implementation.

## Requirements

- Qt (I've designated 2.14 in `main.qml` as it was the version installed on my PC but it should work on any versions)
- Rust
- GStreamer
- some Cargo crates

No need to install/use `rust-qt-binding-generator`, this repo also comes with those bindings (`Bindings.*`). The directory structure is based on [qt_quick_cargo template](https://github.com/KDE/rust-qt-binding-generator/tree/master/templates/qt_quick_cargo).

## Build & Run

1. Clone this repo
2. Before building, you need to do modify a path to video in `setup()` of `main.rs`
  - which is `file:///usr/share/big-buck-bunny_trailer.webm` by default
  - yes I'm gonna fix this
3. `$ cargo build` or `$ cargo run`. 