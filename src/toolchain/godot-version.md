<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Selecting a Godot version

Supporting multiple Godot versions is a key feature of gdext. Especially if you plan to share your extension with others (as a library or an
editor plugin), this page elaborates your choices and their trade-offs in detail. The previous chapter about [compatibility][compat] is
expected as a prerequisite.


## Table of contents

<!-- toc -->


## Motivation

To refresh, you have two Godot versions to consider:

- **API version**, against which gdext compiles.
  - Affects Rust symbols (classes, methods, etc.) you have available at compile time.
  - This sets a lower bound on the Godot binary you can run your extension in.
  
- **Runtime version**, the Godot engine version, in which you run the Rust extension.
  - Affects the runtime behavior, e.g. newer versions may fix some bugs.
  - It is advised to stay up-to-date with Godot releases, to benefit from new improvements.

GDExtension is designed to be backward-compatible, so an extension built with a certain API version can be run in all Godot binaries greater
than that version.[^compat-4-0] Therefore, the lower your API version, the more Godot versions you support.

```admonish abstract title="In other words:"
API version <= runtime version
```

  
### Why support multiple versions?

The choice you have in the context of gdext is the **API version**. If you just make a game on your own, the defaults are typically good enough.

Explicitly selecting an API version can be helpful in the following scenarios:

1. You run/test your application on different Godot **minor** versions.
2. You are collaborating in a team, or you want to give your Godot project to friends to experiment with.
3. You work on a library or plugin to share with the community, either open-source (distributed as code) or closed-source (distributed as
   compiled dynamic library).

Especially in the last case, you may want your extension to be compatible with as many Godot versions as possible, to reach a broader audience.

```admonish tip title="Building an ecosystem"
At first glance, it may not seem obvious why a plugin would support anything but the latest Godot version. After all, users can just update,
right?

However, sometimes users cannot update their Godot version due to regressions, incompatibilities or project/company constraints.

Furthermore, imagine you want to use two GDExtension plugins: **X** (API level 4.3) and **Y** (4.2). Unfortunately, Y contains a bug that 
causes some issues with Godot 4.3. This means you cannot use both together, and you are left with some suboptimal choices:
- Only use X on 4.3.
- Only use Y on 4.2.
- Help the author of Y to patch the bug. But they may just sail the Caribbean and not respond on their repo. Or worse, Y might even be a
  closed-source plugin that you paid for.

Not only are you now left with a less-than-ideal situation, but you cannot build _your own tool_ Z which uses both X and Y, either.
Had X declared API 4.2, people could stick to that version until Y is fixed, and you too could release Z with API 4.2.

A longer compatibility range gives users more flexibility regarding _when_ they update _what_. It accounts for the fact that developers
iterate at varying pace, and enables projects to depend on each other. At scale, this enables a vibrant ecosystem of extensions around Godot.
```


### Cutting edge vs. compatibility

Lower API versions allow supporting a wider range of Godot versions. For example, if you set the API version to 4.2, you can run it in Godot
4.2, 4.2.2 or 4.3, but not Godot 4.1.

On the flip side, lower API versions reduce the API surface that you can statically[^dynamic-features] use in your Rust extension. If you
select 4.2, you will not see classes and functions introduced in 4.3.

This is the core trade-off, and you need to decide based on your use case. If you are unsure, you can always start with a conservatively low API
version, and bump it when you find yourself needing more recent features.


## Selecting the API version in gdext

Now that the _why_ part is clarified, let's get into _how_ you can choose the API version in gdext.


### Default version

By default, gdext uses the **current minor release** of Godot 4, with patch 0. This ensures that it can be run with all Godot patch versions
for that minor release.

Example: if the current release is Godot 4.3.5, then gdext will use API version 4.3.0.


### Lower minor version

To change the API level to a lower version, simply turn on the Cargo feature `api-4-x`, where `x` is the minor version you want to target.

Example in Cargo.toml:

```toml
[dependencies]
# API level 4.2
godot = { ..., features = ["api-4-2"] }
```

You can also explicitly set the current minor version (the same as the default). This has the advantage that you keep that compatibility,
even once gdext starts targeting a newer version by default.

```admonish note title="Mutual exclusivity"
Only one `api-*` feature can be active at any time.
```


### Lower or higher patch version

gdext supports API version granularity on a patch level, if absolutely needed. This is rarely necessary and can cause confusion to users,
so only select a patch-level API if you have a very good reasons. Also note that GDExtension itself is only updated in minor releases.

Reasons to want this might be:

- Godot ships a bugfix in a patch version that is vital for your extension to function properly.
- A new API is introduced in a patch version, and you would like its class/function definitions. This happens quite rarely.

To require a minimum patch level, use a `api-4-x-y` feature:

```toml
[dependencies]
# API level 4.2.1
godot = { ..., features = ["api-4-2-1"] }
```


## Custom Godot versions

If you want to freely choose a Godot binary on your local machine from which the GDExtension API is generated, you can use the Cargo feature
`api-custom`. If enabled, this will look for a Godot binary in two locations, in this order:

1. The environment variable `GODOT4_BIN`.
2. The binary `godot4` in your `PATH`.

Generated code inside the `godot::builtin`, `godot::classes` and `godot::global` modules may now look different from stable releases.
Note that we [do not give any support or compatibility guarantees][no-custom-support] for custom-built GDExtension APIs.

Working with the `api-custom` feature requires the `bindgen` crate, as such you may need to install the LLVM toolchain.
Consult the [setup page][setup-llvm] for more information.


### Setting `GODOT4_BIN` to a relative path

If you have multiple Godot workspaces on a machine, you may want a workspace-independent method of setting the `GODOT4_BIN` environment variable.
This way, the matching Godot editor binary for that workspace is always used in the build process, without having to set `GODOT4_BIN` differently
for each location.

You can do this by configuring Cargo to set `GODOT4_BIN` to a relative path for you, in `.cargo/config.toml`.

In the root of your Rust project, create `.cargo/config.toml` with the example content shown below, modifying the editor path as needed to find
your binary. The path you set will be resolved relatively to the location of the `.cargo` directory.

```toml
[env]
GODOT4_BIN = { value = "../godot/bin/godot.linuxbsd.editor.x86_64", relative = true, force = true }
```

(If you want to override `config.toml` by setting `GODOT4_BIN` in your environment, remove `force = true`.)

Test your change by running `cargo build`.

See [The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html) for more information on customizing your build environment with
`config.toml`.


[api-gdext-build]: https://godot-rust.github.io/docs/gdext/master/godot/init/struct.GdextBuild.html
[compat-guarantees]: compatibility.md#current-guarantees
[compat]: compatibility.md
[no-custom-support]: compatibility.md#out-of-scope
[setup-llvm]: ../intro/setup.md#llvm

---

**Footnotes**

[^compat-4-0]: Godot 4.0 has been released before the GDExtension API committed to stability, so no single 4.0.x release is compatible with any
other release (not even patch versions among each other). We provide 4.0 API levels, but due to their limited utility, we will phase out
support very soon.

[^dynamic-features]: Even if your API level is 4.2, it is possible to access 4.3 features, but you need to do so dynamically. This can be
achieved using reflection APIs like `Object::call()`, but you lose the type safety and convenience of the statically generated API.
To obtain version information, check out the [`GdextBuild` API][api-gdext-build].
