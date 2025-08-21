<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering classes

Classes are the backbone of data modeling in Godot. If you want to build complex user-defined types in a type-safe way, you won't get around
classes. Arrays, dictionaries and simple types only get you so far, and overusing them defeats the purpose of using a statically typed language.

Rust makes class registration straightforward. As mentioned before, Rust syntax is used as a baseline, with godot-rust specific additions.


See also [GDScript reference for classes][godot-gdscript-classes].


## Table of contents

<!-- toc -->


## Defining a Rust struct

In Rust, Godot classes are represented by structs. Structs are defined as usual and can contain any number of fields. To register them with
Godot, you need to derive the `GodotClass` trait.

```admonish info title="GodotClass trait"
The `GodotClass` trait marks all classes known in Godot. It is already implemented for engine classes, for example `Node` or `Resource`.
If you want to register your own classes, you need to implement `GodotClass` as well.

`#[derive(GodotClass)]` streamlines this process and takes care of all the boilerplate.  
See [API docs][api-derive-godotclass] for detailed information.
```

Let's define a simple class named `Monster`:

```rust
#[derive(GodotClass)]
#[class(init)] // more about this later.
struct Monster {
    name: String,
    hitpoints: i32,
}
```

That's it. Immediately after compiling, this class becomes available in Godot through hot reloading (before Godot 4.2, after restart).
It won't be very useful yet, but the above definition is enough to register `Monster` in the engine.

```admonish info title="Auto-registration"
`#[derive(GodotClass)]` _automatically_ registers the class -- you don't need an explicit `add_class()` registration call
or a central list mentioning all classes.

The proc-macro internally registers the class in such a list at startup time.
```


## Selecting a base class

By default, the base class of a Rust class is `RefCounted`. This is consistent with GDScript when you omit the `extends` keyword.

`RefCounted` is quite useful for data bundles. As implied by the name, it allows sharing instances tracked by a reference counter;
as such, you don't need to worry about memory management. `Resource` is a subclass of `RefCounted` and is useful for data that needs to be
serialized to the filesystem.

However, if you want your class to be part of the scene tree, you need to use `Node` (or one of its derived classes) as a base class.

Here, we use a more concrete node type, `Node3D`. This is done by specifying `#[class(base=Node3D)]` on the struct definition:

```rust
#[derive(GodotClass)]
#[class(base=Node3D)]
struct Monster {
    name: String,
    hitpoints: i32,
}
```


## The base field

Since Rust does not have inheritance, we need to use composition to achieve the same effect. godot-rust provides a `Base<T>` type, which lets us
store the instance of the Godot superclass (base class) as a field in our `Monster` class.

```rust
#[derive(GodotClass)]
#[class(base=Node3D)]
struct Monster {
    name: String,
    hitpoints: i32,
    base: Base<Node3D>,
}
```

The important part is the `Base<T>` type. `T` must match the base class you specified in the `#[class(base=...)]` attribute.
You can also use the associated type `Self::Base` for `T`.

When you declare a base field in your struct, the `#[derive]` procedural macro will automatically detect the `Base<T>` type.[^inference]
This lets you access the `Node` API through provided methods `self.base()` and `self.base_mut()`, but more on this later.


## Conclusion

You have learned how to define a Rust class and register it with Godot. You now know that different base classes exist and how to select one.

The next chapters cover functions and constructors.

<br>

---

[^inference]: You can tweak the type detection using the `#[hint]` attribute, see [the corresponding docs][api-derive-godotclass-inference].


[api-derive-godotclass]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html
[api-derive-godotclass-inference]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#fine-grained-inference-hints
[api-godot-api]: https://godot-rust.github.io/docs/gdext/master/godot/register/attr.godot_api.html
[godot-gdscript-classes]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#classes
