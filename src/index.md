<!--
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Introduction

Welcome to the **godot-rust book**! This is a work-in-progress user guide for **gdext**, the Rust binding for Godot 4.

If you're new to Rust, before getting started, it is highly recommended that you familiarize yourself with concepts outlined
in the officially maintained [Rust Book](https://doc.rust-lang.org/book/).

To read the book about gdnative (Godot 3 binding), follow [this link](../gdnative-book).


## Terminology

To avoid confusion, here is an explanation of names and technologies you may encounter over time.

* **godot-rust**: The entire project, encompassing Rust bindings for Godot 3 and 4, as well as related efforts (book, community, etc.).
* [**GDExtension**]: C API provided by Godot 4.
* [**GDNative**]: C API provided by Godot 3.
* **gdext** (lowercase): the Rust binding for GDExtension (Godot 4) -- what this book focuses on.
* **gdnative** (lowercase): the Rust binding for GDNative (Godot 3).
* **Extension**: An extension is a C library developed using gdext. It can be loaded by Godot 4.


## Currently supported features

For an up-to-date overview of implementation status, consult [issue #24][features].


## GDExtension API: what's new

This section briefly mentions the difference between the native interfaces in Godot 3 and 4 from a functional point of view,
without going into Rust.

While the underlying FFI (foreign function interface) layer has been completely rewritten, a lot of concepts remain the same
from a user point of view. In particular, Godot's approach with a node-based scene graph, composed of classes in an inheritance
relation, has not changed.

That said, there are some notable differences:

1. **No more native scripts**

   With GDNative, Rust classes could be registered as _native scripts_. These scripts are attached to nodes in order to enhance
   their functionality, analogous to how GDScript scripts could be attached. GDExtension on the other hand directly supports Rust types
   as engine classes, see also next point.

   Keep this in mind when porting GDScript code to Rust: instead of replacing the GDScript with a native script, you need to change the
   node type to a Rust class that inherits the node.

2. **First-class citizen types**

   In Godot 3, user-defined native classes had lots of limitations in the editor: type annotations were not fully supported, they could
   not easily be used as custom resources, etc. With GDExtension, user-defined classes in Rust behave much closer to GDScript classes.

3. **Always-on**

   There is no differentiation between "tool" and "normal" scripts anymore, as it was the case in GDNative. Rust logic runs as soon as
   the Godot editor launches, but gdext explicitly changes this behavior. By default, all virtual callbacks (`ready`, `process` etc.)
   are not invoked **in editor mode**. This behavior can be configured when implementing the [`ExtensionLibrary`][extension-library-doc] trait.

4. **No recompilation while editor is open**

   Prior to Godot 4.2, it was not possible to recompile a Rust library and let changes take effect when the game is launched from the editor.
   This has recently been implemented though, see [issue #66231].


[features]: https://github.com/godot-rust/gdextension/issues/24
[issue #66231]: https://github.com/godotengine/godot/issues/66231
[extension-library-doc]: https://godot-rust.github.io/docs/gdext/master/godot/init/trait.ExtensionLibrary.html#method.editor_run_behavior

[**GDNative**]: https://docs.godotengine.org/en/3.5/tutorials/scripting/gdnative/what_is_gdnative.html
[**GDExtension**]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html
