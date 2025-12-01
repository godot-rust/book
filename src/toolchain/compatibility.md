<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Compatibility and stability

The godot-rust library supports multiple stable Godot releases at a time.

<!-- toc -->


## Compatibility with Godot

When developing extension libraries (or just "extensions"), you need to consider which engine version you want to target.
There are two conceptually different versions:

- **API version** is the version of GDExtension against which your extension is **compiled**.
- **Runtime version** is the version of Godot in which the library built with godot-rust is **run**.

The two versions can be different, with some constraints elaborated below.


### Current guarantees

Latest godot-rust requires at least **Godot 4.2**.

Starting from that version's official release, extensions can be loaded by any Godot version, as long as _runtime version **>=** API version_.
In other words, you can run existing extensions in newer Godot versions without needing to change anything.

- You **can** run a `4.2` extension in Godot `4.2.1` or `4.3`.
- You **cannot** run a `4.3` extension in Godot `4.2.1`.

As long as the GDExtension API evolves in a backward-compatible manner -- which it has remarkably achieved since Godot 4.1 -- we will try our
best to keep up this guarantee. If you notice any discrepancies, please report them to us.


### Compatibility matrix

We typically provide support for Godot versions for 1-2 years after their release, depending on feature set and maintenance effort.
For example, Godot 4.0 extensions are binary-incompatible with newer versions and thus provide very little value.
Godot 4.1 also lacks foundational features necessary for Rust callables, typed signals, hot reloading and much more.

If you need to support an older Godot version, you can fall back to older godot-rust releases.
These won't receive any more updates however, not even for critical bugs.

| godot-rust version | minimum Godot version | Godot release date[^Godot-versions] |
|--------------------|-----------------------|-------------------------------------|
| 0.4+               | 4.2                   | November 2023                       |
| 0.2, 0.3           | 4.1                   | July 2023                           |
| 0.1                | 4.0[^Godot-4-0]       | March 2023                          |

Make sure to use the appropriate `api-4-*` feature flag, see [_Selecting a Godot version_](godot-version.md).


### Philosophy

We take compatibility with the engine seriously, in an attempt to build an ecosystem of extensions that are interoperable with multiple
Godot versions. Nothing is more annoying than updating the engine and recompiling 10 plugins/extensions.

This is sometimes difficult, because:

- Godot may introduce subtle breaking changes of which we are not aware.
- Some changes that are non-breaking in C++ and GDScript are breaking in Rust (e.g. providing a default value for a previously required parameter).
- Using newer features needs to come with a fallback/polyfill for older Godot versions.

We run CI jobs against multiple Godot versions, to get a certain level of confidence that updates do not break compatibility.
Nevertheless, the number of possible combinations is large and only growing, so we may miss certain issues.
If you find incompatibilities or violations of the rules stated below, please let us know.


### Out of scope

We do **not** invest effort in maintaining compatibility with:

1. Godot in-development versions, except for the latest `master` branch.
   - Note that we may take some time to catch up with the latest changes, so please don't report issues within a few days after
     upstream changes have landed.

2. Non-stable releases (alpha, beta, RC).
3. Third-party bindings or GDExtension APIs (C#, C++, Python, ...).
   - These may have their own versioning guarantees and release cycles; and there may be specific bugs to such an integration.
     If you find an issue with godot-rust and another binding, reproduce it in GDScript to make sure it's relevant for us.
   - We do however maintain compatibility with Godot, so if integrations go through the engine (e.g. Rust calls a method whose
     implementation is in C#), this should work.
4. Godot with non-standard build flags (e.g. disabled modules).
5. Godot forks or engines running third-party modules.


## Rust API stability

A lot of godot-rust's foundation has been be built and is in a production-ready state. However, we still regularly add new features, and
sometimes refine existing APIs.

As such, **expect occasional breaking changes**. These are usually minor and will be announced in both [changelog] and
[migration guides][migrate]. We additionally work with deprecations in our API, allowing smooth transitions.

Note that if breaking changes occur, they are externally motivated, for example:

- GDExtension changes in a way that cannot be abstracted from the user.
- There are subtleties in the type system or runtime guarantees that can be modeled in a better, safer way (e.g. typed arrays, RIDs).
- We get feedback from game developers and other users stating that certain workflows are very cumbersome.

Our [crates.io releases](https://crates.io/crates/godot) adhere to SemVer, but may lag behind the master branch.


[changelog]: https://github.com/godot-rust/gdext/blob/master/Changelog.md
[migrate]: https://godot-rust.github.io/book/migrate


<br>

---

**Footnotes**

[^Godot-versions]: See _Release history_ on [Wikipedia](https://en.wikipedia.org/wiki/Godot_(game_engine)#Release_history).

[^Godot-4-0]: Every extension developed with API version `4.0.x` **MUST** be run with the same runtime version.
    In particular, it is not possible to run an extension compiled with API version `4.0.x` in Godot 4.1 or later.
    This is due to breaking changes in Godot's GDExtension API.
