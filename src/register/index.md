<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering Rust symbols

This chapter teaches how you make your own Rust code available to Godot. You do this by _registering_ individual symbols (classes, functions etc.)
in the engine.

Starting with class registration, the chapter then goes into the details of registering functions, properties, signals and constants.

<!-- TODO: Futher aspects cover the Rust-to-Godot conversions using `ToGodot`/`FromGodot` traits and the registration of enums. -->


## Proc-macro API

The proc-macro API is currently the only way to register Rust symbols. A variety of procedural macros (derive and attribute macros) are provided
to decorate your Rust items, such as `struct`s or `impl` blocks. Behind the scenes, these macros generate the necessary glue code to register
each item with Godot.

The library is designed in a way that you can use all your existing knowledge and simply extend it with macro syntax, rather than having to learn
a completely new way of doing things. We try to avoid foreign DSLs (domain-specific languages) and instead build on top of Rust's existing syntax.

This approach does a respectable job at limiting the amount of boilerplate code you have to write, and thus makes it much easier for you to
focus on the important bits. For example, you will rarely have to repeat yourself more than necessary or register one thing in multiple places
(e.g. declare a method, mention it in another `register` method and then repeat its name yet again as a string literal).


## "Exporting"

The term "exporting" is sometimes erroneously used. Please avoid talking about "exporting classes" or "exporting methods" if you mean
"registering". This can often cause confusion, especially among beginners.

_Export_ already has two well-defined meanings in the context of Godot:

1. Exporting a property. This does not _register_ the property with Godot, but renders it visible in the editor.
   - GDScript uses the `@export` annotation for this, we use `#[export]`.
   - See also [GDScript exported properties][godot-export-properties].

2. Exporting projects, meaning bundling them for release.
   - The editor provides a UI to build release versions of your game or application, so they can run as a standalone executable.
     This process of building the executable is called "exporting".
   - See also [Exporting projects][godot-export-projects].

[godot-export-properties]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_exports.html
[godot-export-projects]: https://docs.godotengine.org/en/stable/tutorials/export/index.html
