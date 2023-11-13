<!--
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Setup

To use gdext, we need a few technologies.


## Godot Engine

While you can write Rust code without having the Godot engine, we highly recommend to install Godot for quick feedback loops.
For the rest of the tutorial, we assume that you have Godot 4 installed and available either:

- in your `PATH` as `godot4`,
- or an environment variable called `GODOT4_BIN`, containing the path to the Godot executable.


### Godot from pre-built binaries

Binaries of Godot 4 can be downloaded [from the official website][godot-download].
For beta and older versions, you can also check the [Archive][godot-download-archive].


### Installing Godot via command-line

```bash
# --- Linux ---
# For Ubuntu or Debian-based distros
apt install godot

# For Fedora/RHEL
dnf install godot

# Distro-independent through Flatpak
flatpak install flathub org.godotengine.Godot


# --- Windows ---
# Windows installations can be made through WinGet
winget install --id=GodotEngine.GodotEngine -e


# --- macOS ---
brew install godot
```

```admonish note
If you plan to target Godot versions different from the latest stable release, please read [Compatibility and stability][compatibility].
```


## Rust

[rustup] is the recommended way to install the Rust toolchain, including the compiler, standard library and Cargo (the package manager).
This page contains installation instructions for your platform.


### Installing Rustup via command-line

```bash
# Linux (distro-independent)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
winget install --id=Rustlang.Rustup  -e

# macOS
brew install rustup
```

After installation of rustup and the `stable` toolchain, check that they were installed properly:

```bash
# Check Rust toolchain installer version
rustup -V
# Check Rust version
rustc --version
# Check Cargo version
cargo -V
```

If you use Windows, check out [Working with Rust on Windows][rustup-windows].


## LLVM

In general, you do **NOT need to install LLVM.**


This was necessary in the past due to `bindgen`, which [depends on LLVM][llvm-bindgen].
However, we now provide pre-built artifacts, so that most users can simply add the Cargo dependency and start immediately.
This also significantly reduces initial compile times, as `bindgen` was quite heavyweight with its many transitive dependencies.

You will still need LLVM if you plan to use the `custom-godot` feature, for example if you have a forked version of Godot or custom
modules (but not if you need just a different API version; see [Selecting a Godot version][godot-version]).

You may download LLVM binaries from [llvm.org][llvm].

After installation, check that LLVM was installed properly:

```bash
# Check if Clang is installed and registered in PATH
clang -v
```


[compatibility]: ../advanced/compatibility.md
[godot-download]: https://godotengine.org/download/
[godot-download-archive]: https://godotengine.org/download/archive/
[godot-version]: ../advanced/godot-version.md
[llvm-bindgen]: https://rust-lang.github.io/rust-bindgen/requirements.html
[llvm]: https://releases.llvm.org
[rustup-windows]: https://github.com/rust-lang/rustup#working-with-rust-on-windows
[rustup]: https://rustup.rs
