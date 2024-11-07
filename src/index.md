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

To read the book about gdnative (Godot 3 binding), follow [this link](../gdnative-book).


## The purpose of godot-rust

Godot is a batteries-included game engine that fosters a productive and fun gamedev workflow. It ships GDScript as a built-in scripting
language and also provides official support for C++ and C# bindings. Its GDExtension mechanism allows more languages to be integrated,
in our case Rust.

Rust brings a modern, robust and performant experience to game development. If you are interested in scalability, strong type systems or
just enjoy Rust as a language, you may consider using it with Godot, to combine the best of both worlds.


## About this project

godot-rust is a [community-developed][github-contributors] open source project. It is maintained independently of Godot itself, but we are in
close contact with engine developers, to foster a steady exchange of ideas. This has allowed us to address a lot of Rust's needs upstream, but
also led to improvements of the engine itself in several cases.


### Currently supported features

For an up-to-date overview of implementation status, consult [issue #24][features].


### Terminology

To avoid confusion, here is an explanation of names and technologies you may encounter over the course of this book:

- [**godot-rust**][ref-godot-rust]: The entire project, encompassing Rust bindings for Godot 3 and 4,
  as well as related efforts (book, community, etc.).
- [**GDExtension**][ref-godot-gdext]: C API provided by Godot 4.
- [**GDNative**][ref-godot-gdnative]: C API provided by Godot 3.
- [**gdext**][github-gdext] (lowercase): the Rust binding for GDExtension (Godot 4) -- what this book focuses on.
- [**gdnative**][github-gdnative] (lowercase): the Rust binding for GDNative (Godot 3).
- **Extension**: An extension is a dynamic C library, developed by any language binding (Rust, C++, Swift, ...). It uses the GDExtension API and can
  be loaded by Godot 4.


### GDExtension API: what's new

This section briefly mentions the difference between the native interfaces in Godot 3 and 4 from a functional perspective.

While the underlying FFI (foreign function interface) layer has been completely rewritten, a lot of concepts remain the same from a user point of
view. In particular, Godot's approach with a node-based scene graph, composed of classes in an inheritance relation, has not changed.

That said, there are some notable differences:

1. **Native scripts ⇾ extension classes**

   With GDNative, Rust classes could be registered as _native scripts_. These scripts are attached to nodes in order to enhance
   their functionality, analogous to how GDScript scripts could be attached. GDExtension on the other hand directly supports Rust types
   as engine classes, see also next point.

   When porting GDScript code to Rust, keep in mind that the first-class way to use Rust code is via classes, not scripts. Thanks to
   great contributions, we _do_ in the meantime support [Rust scripts][api-obj-script], albeit less developed than classes.

2. **First-class citizen types**

   In Godot 3, user-defined native classes had lots of limitations in the editor: type annotations were not fully supported, they could
   not easily be used as custom resources, etc. With GDExtension, user-defined classes in Rust behave much closer to GDScript classes.
   They also no longer need a separate `.gdns` file to be registered.

3. **Always-on**

   GDNative had the differentiation between "tool" and "normal" scripts. In GDExtension, native logic by default runs as soon as the Godot editor
   launches, but godot-rust explicitly changes this behavior. In Rust, all virtual callbacks (`ready`, `process` etc.) are not invoked
   **in editor mode**. This behavior can be configured with `#[class(tool)]` and the [`ExtensionLibrary`][extension-library-doc] trait.

4. **No recompilation while editor is open**

   Prior to Godot 4.2, it was not possible to recompile a Rust library and let changes take effect when the game is launched from the editor.
   Editor reloading has since been implemented though, see [issue #66231].


[features]: https://github.com/godot-rust/gdextension/issues/24
[issue #66231]: https://github.com/godotengine/godot/issues/66231
[extension-library-doc]: https://godot-rust.github.io/docs/gdext/master/godot/init/trait.ExtensionLibrary.html#method.editor_run_behavior

[api-obj-script]: https://godot-rust.github.io/docs/gdext/master/godot/obj/script/index.html
[github-contributors]: https://github.com/godot-rust/gdext/graphs/contributors
[github-gdext]: https://github.com/godot-rust/gdext
[github-gdnative]: https://github.com/godot-rust/gdnative
[ref-godot-gdext]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html
[ref-godot-gdnative]: https://docs.godotengine.org/en/3.5/tutorials/scripting/gdnative/what_is_gdnative.html
[ref-godot-rust]: https://godot-rust.github.io/
