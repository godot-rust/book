<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Selecting a Godot version

By default, `gdext` uses the latest stable release of Godot. This is desired in most cases, but it means that you cannot run your extension in
an older Godot version. Furthermore, you cannot benefit from modified Godot versions (e.g. with custom modules).

If these are features you need, this page will walk you through the necessary steps.
Read [Compatibility and stability] first and make sure you understand the concept of API and runtime versions.


## Older stable releases

Building gdext against an older Godot API allows you to remain forward-compatible with all engine versions >= that version.
(For Godot 4.0.x, [== applies instead of >=][compat-guarantees].)

In a hypothetical example, building against API 4.1 allows you to run your extension in Godot 4.1.1, 4.1.2 or 4.2.

To choose a version (here `4.0`), add the following to your top-level (workspace) `Cargo.toml`:

```toml
[patch."https://github.com/godot-rust/godot4-prebuilt".godot4-prebuilt]
git = "https://github.com//godot-rust/godot4-prebuilt"
branch = "4.0"
```

(If you're interested in the `//` workaround, see <https://github.com/rust-lang/cargo/issues/5478>).


## Custom Godot versions

If you want to freely choose a Godot binary on your local machine from which the GDExtension API is generated, you can use the Cargo feature
`custom-godot`. If enabled, this will look for a Godot binary in two locations, in this order:

1. The environment variable `GODOT4_BIN`.
2. The binary `godot4` in your `PATH`.

Generated code inside the `godot::engine` and `godot::builtin` modules may now look different from stable releases.
Note that we do not give any support or compatibility guarantees for custom-built GDExtension APIs.

Note that this requires the `bindgen`, as such you may need to install the LLVM toolchain.
Consult the [setup page][setup-llvm] for more information.


### Setting `GODOT4_BIN` to a relative path

If you have multiple Godot workspaces on a machine, you may want a workspace-independent method of setting the `GODOT4_BIN` environment variable.
This way, the matching Godot editor binary for that workspace is always used in the build process, without having to set `GODOT4_BIN` differently for each
location.

You can do this by configuring Cargo to set `GODOT4_BIN` to a relative path for you, in `.cargo/config.toml`.

In the root of your Rust project, create `.cargo/config.toml` with the example content shown below, modifying the editor path as needed to find
your binary. The path you set will be resolved relative to the location of the `.cargo` directory.

```toml
[env]
GODOT4_BIN = { value = "../godot/bin/godot.linuxbsd.editor.x86_64", relative = true, force = true }
```

(If you want to override `config.toml` by setting `GODOT4_BIN` in your environment, remove `force = true`.)

Test your change by running `cargo build`.

See [The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html) for more information on customizing your build environment with
`config.toml`.


[Compatibility and stability]: compatibility.md
[compat-guarantees]: compatibility.md#current-guarantees
[setup-llvm]: ../intro/setup.md#llvm

