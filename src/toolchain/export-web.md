<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Export to Web

Web builds are a fair bit more difficult to get started with compared to native builds.
This will be a complete guide on how to get things compiled.
However, setting up a web server to host and share your game is considered out of scope of this guide, and is best explained elsewhere.

```admonish warning
Web support with gdext is experimental and should be understood as such before proceeding.
```


## Installation

Install a nightly build of `rustc`, the `wasm32-unknown-emscripten` target for `rustc`, and `rust-src`.
The reason why nightly `rustc` is required is the unstable flag to build `std` ([`-Zbuild-std`][flag-build-std]).
Assuming that Rust was installed with `rustup`, this is quite simple.


  ```sh
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  rustup target add wasm32-unknown-emscripten --toolchain nightly
  ```

Next, install Emscripten.  The simplest way to achieve this is to install [`emsdk` from the git repo][emsdk-git].
We recommended version 3.1.39 for now.[^1]

```sh
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install 3.1.39
./emsdk activate 3.1.39
source ./emsdk.sh     (or ./emsdk.bat on windows)
```

It would also be **highly** recommended to follow the instructions in the terminal to add `emcc`[^2] to your `PATH`.
If not, it is necessary to manually `source` the `emsdk.sh` file in every new terminal prior to compilation.
This is platform-specific.

[flag-build-std]: https://doc.rust-lang.org/cargo/reference/unstable.html#list-of-unstable-features
[emsdk-git]: https://github.com/emscripten-core/emsdk#readme


## Project Configuration

Enable the [`experimental-wasm`][api-cargo-features] feature on gdext in the `Cargo.toml` file.
It is also recommended to enable the [`lazy-function-tables`][api-cargo-features] feature to avoid long compile times with release builds
(this might be a bug and not necessary in the future). Edit the line to something like the following:

```toml
[dependencies.godot]
git = "https://github.com/godot-rust/gdext"
branch = "master"
features = ["experimental-wasm", "lazy-function-tables"]
```

If you do not already have a `.cargo/config.toml` file, do the following:

- Create a `.cargo` directory at the same level as your `Cargo.toml`.
- Inside that directory, create a `config.toml` file.

This file needs to contain the following:

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
    "-C", "link-args=-sSIDE_MODULE=2",
    "-C", "link-args=-pthread", # was -sUSE_PTHREADS=1 in earlier emscripten versions
    "-C", "target-feature=+atomics,+bulk-memory,+mutable-globals",
    "-Zlink-native-libraries=no",
    "-Clink-arg=-fwasm-exceptions",
    "-Clink-args=-sDISABLE_EXCEPTION_CATCHING=1",
    "-Clink-args=-sEXPORT_ALL=1",
    "-Clink-args=-sSUPPORT_LONGJMP=wasm",
    "-Cllvm-args=-enable-emscripten-cxx-exceptions=0",
    "-Cllvm-args=-wasm-enable-sjlj",
]
```

Edit the project's `.gdextension` file to include support for web exports.
This file will probably be at `godot/{YourCrate}.gdextension`.
The format will be similar to the following:

```ini
[libraries]
...
web.debug.wasm32 = "res://../rust/target/wasm32-unknown-emscripten/debug/{YourCrate}.wasm"
web.release.wasm32 = "res://../rust/target/wasm32-unknown-emscripten/release/{YourCrate}.wasm"
```

[api-cargo-features]: https://godot-rust.github.io/docs/gdext/master/godot/#cargo-features


## Compile the Project

Verify `emcc` is in the `PATH`. This can be as simple as doing the following:

```sh
emcc --version
```

Compile the code.
It is necessary to both use the nightly compiler and specify to build std[^3], along with specifying the Emscripten target.

```sh
cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
```


## Godot editor setup

Add a web export in the Godot Editor. In the top menu bar, go to `Project > Export...` and configure it there.
Make sure to turn on the `Extensions Support` checkbox.

![Example of export screen](images/web-export.png)

If instead, the bottom on the export popup contains this error in red:

> No export template found at expected path:

Then click on `Manage Export Templates` next to the error message, and then on the next screen select `Download and Install`.
See [Godot tutorial][godot-export-templates] for further information.


### Running the webserver

Back at the main editor screen, there is an option to run the web debug build (_not_ a release build) locally
without needing to run an export or set up a web server.
At the top right, choose `Remote Debug > Run in Browser` and it will automatically open up a web browser.

![Location of built-in web server](images/web-browser-run.png)


```admonish warning title="Known Caveats"
- Godot 4.1.3+ or 4.2+ is necessary.
- Only Chromium-based browsers (Chrome or Edge) appear to be supported by GDExtension at the moment; Firefox and Safari don't work yet.
  Info about browser support can be found [here](https://github.com/godotengine/godot-cpp/pull/1247#issuecomment-1742197814).
```

If your default browser is not Chromium-based, you will need to copy the URL (which is usually `http://localhost:8060/tmp_js_export.html`)
and open it in a supported browser such as Google Chrome or Microsoft Edge.

[godot-export-templates]: https://docs.godotengine.org/en/stable/tutorials/export/exporting_projects.html#export-menu


## Debugging

Currently, the only option for WASM debugging is
[this extension](https://chromewebstore.google.com/detail/cc++-devtools-support-dwa/pdcpmagijalfljmkmjngeonclgbbannb?pli=1)
for Chrome. It adds support for breakpoints and a memory viewer into the F12 menu.


<br>

---

[^1]: Note: Due to a bug with `emscripten`, the maximum version of `emcc`[^2] that can one compile `Godot` with is `3.1.39`.  gdext itself should be able to support the latest version of `emcc`, however, it may be a safer bet to stick to version `3.1.39`.

[^2]: `emcc` is the name of Emscripten's compiler.

[^3]: The primary reason for this is it is necessary to compile with `-sSHARED_MEMORY` enabled. The shipped `std` does not, so building `std` is a requirement. Related info on about WASM support can be found [here](https://github.com/rust-lang/rust/issues/77839).

