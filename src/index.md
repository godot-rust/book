<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Introduction

Welcome to the **godot-rust book**! This is a work-in-progress user guide for **gdext**, the Rust binding for Godot 4.

If you're new to Rust, before getting started, it is highly recommended that you familiarize yourself with concepts outlined in the officially
maintained [Rust Book](https://doc.rust-lang.org/book/).

Additional resources that may be interesting for you:

📘 [Latest API docs][api-docs]  
⚗️ [Demo projects][demo-projects]  
🀄 [This book in Simplified Chinese][book-zh-cn]  
📔 [Book on gdnative (Godot 3 binding)][gdnative-book]  


## The purpose of godot-rust

Godot is a batteries-included game engine that fosters a productive and fun gamedev workflow. It ships GDScript as a built-in scripting
language and also provides official support for C++ and C# bindings. Its GDExtension mechanism allows more languages to be integrated,
in our case Rust.

Rust brings a modern, robust and performant experience to game development. If you are interested in scalability, strong type systems or
just enjoy Rust as a language, you may consider using it with Godot, to combine the best of both worlds.

See also [Philosophy][philosophy] to learn more about the core ideas behind godot-rust.


## About this project

godot-rust is a [community-developed][github-contributors] open source project. It is maintained independently of Godot itself, but we are in
close contact with engine developers, to foster a steady exchange of ideas. This has allowed us to address a lot of Rust's needs upstream, but
also led to improvements of the engine itself in several cases.


### Currently supported features

For an up-to-date overview of implementation status, consult [issue #24][features].


### Terminology

To avoid confusion, here is an explanation of names and technologies you may encounter over the course of this book:

- [**godot-rust**][ref-godot-rust]: The Godot 4 Rust bindings, and sometimes also the entire project (book, community, website etc.).
- [**gdext**][github-gdext] (lowercase): explicitly just Rust 4 bindings. We prefer the term "godot-rust" which is more recognizable
  outside this specific ecosystem.
- [**gdnative**][github-gdnative] (lowercase): the Rust binding for GDNative (Godot 3), no longer actively maintained.
- [**GDExtension**][ref-godot-gdext]: C API provided by Godot 4.
- [**GDNative**][ref-godot-gdnative]: C API provided by Godot 3.
- **Extension**: An extension is a dynamic C library, developed by any language binding (Rust, C++, Swift, ...). It uses the GDExtension API and can
  be loaded by Godot 4.

These are _WRONG_ terms: `GDRust`, `gdrust`, `godot-rs`.


[features]: https://github.com/godot-rust/gdextension/issues/24

[api-docs]: https://godot-rust.github.io/docs/gdext
[book-zh-cn]: https://colinwttt.github.io/godot-rust-book-chinese
[demo-projects]: https://github.com/godot-rust/demo-projects
[gdnative-book]: https://godot-rust.github.io/gdnative-book
[github-contributors]: https://github.com/godot-rust/gdext/graphs/contributors
[github-gdext]: https://github.com/godot-rust/gdext
[github-gdnative]: https://github.com/godot-rust/gdnative
[ref-godot-gdext]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html
[ref-godot-gdnative]: https://docs.godotengine.org/en/3.5/tutorials/scripting/gdnative/what_is_gdnative.html
[ref-godot-rust]: https://godot-rust.github.io/
[philosophy]: contribute/philosophy
