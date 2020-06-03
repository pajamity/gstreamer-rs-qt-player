# Rust + Qt + GStreamer example

Simple video player made on top of GStreamer + Qt + Rust with the help of [gstreamer-rs]([gstreamer-rs](https://github.com/sdroege/gstreamer-rs)), [rust-qt-bindings-generator](https://github.com/KDE/rust-qt-binding-generator). I also referrerd to [QtQuickPlayer](https://github.com/ivasilev/QtQuickPlayer) and [Quemail](https://gitlab.com/rhn/quemail) to figure out how they manage to integrate Qt+GStreamer and Qt+Rust, respectively.

This is my learning project. Feel free to make PRs and issues if you have better ideas of implementation.

## Requirements

- Qt (I've designated 2.14 in `main.qml` as it was the version installed on my PC but it should work in older versions)
- Rust
- GStreamer
- some Cargo crates

## Build & Run

``$ cargo build`` or ``$ cargo run``. No need to install `rust-qt-binding-generator`, this repo also comes with those bindings (`Bindings.*`).