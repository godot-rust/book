<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Setup

Before we can start writing Rust code, we need to install a few tools.


## Godot Engine

While you can write Rust code without the Godot engine, we highly recommend to install Godot for quick feedback loops.
For the rest of the tutorial, we assume that you have Godot 4 installed and available either:

- in your `PATH` as `godot4`,
- or an environment variable called `GODOT4_BIN`, containing the path to the Godot executable.


### Godot from pre-built binaries

Binaries of Godot 4 can be downloaded [from the official website][godot-download].  
For beta and older versions, you can also check the [download archive][godot-download-archive].


### Installing Godot via command-line

```bash
# --- Linux ---
# For Ubuntu or Debian-based distros.
apt install godot

# For Fedora/RHEL.
dnf install godot

# Distro-independent through Flatpak.
flatpak install flathub org.godotengine.Godot


# --- Windows ---
# Windows installations can be made through WinGet.
winget install --id=GodotEngine.GodotEngine -e


# --- macOS ---
brew install godot
```

```admonish note title="Other Godot versions"
If you plan to target Godot versions different from the latest stable release, please read [Selecting a Godot version][godot-version].
```


## Rust

[rustup] is the preferred way to install the Rust toolchain. It includes the compiler, standard library, Cargo (the package manager)
as well as tools like rustfmt or clippy. Visit the website to download binaries or installers for your platform. Alternatively, you can
install it via command-line.


### Installing rustup via command-line

```bash
# Linux (distro-independent)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
winget install --id=Rustlang.Rustup -e

# macOS
brew install rustup
```

After installation of rustup and the `stable` toolchain, you can verify that they are working:

```bash
$ rustc --version
rustc 1.74.1 (a28077b28 2023-12-04)
```


## LLVM

```admonish tip
In general, you do **NOT** need to install LLVM.
```

This was necessary in the past due to `bindgen`, which [depends on LLVM][llvm-bindgen].
However, we now provide pre-built artifacts, so that most users can simply add the Cargo dependency and start immediately.
This also significantly reduces initial compile times, as `bindgen` was quite heavyweight with its many transitive dependencies.

You will still need LLVM if you plan to use the `api-custom` feature, for example if you have a forked version of Godot or custom
modules. To just use a different API version of Godot, you do _not_ need LLVM though; see [Selecting a Godot version][godot-version].

LLVM binaries can be downloaded from [llvm.org][llvm]. Once installed, you can check whether LLVM's clang compiler is available:

```bash
clang -v
```


[godot-download-archive]: https://godotengine.org/download/archive/
[godot-download]: https://godotengine.org/download/
[godot-version]: ../toolchain/godot-version.md
[llvm-bindgen]: https://rust-lang.github.io/rust-bindgen/requirements.html
[llvm]: https://releases.llvm.org
[rustup-windows]: https://github.com/rust-lang/rustup#working-with-rust-on-windows
[rustup]: https://rustup.rs
