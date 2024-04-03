<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Objects

This chapter covers the most central mechanism of the Rust bindings -- one that will accompany you from the Hello-World
example to a sophisticated Rust game.

We're talking about _objects_ and the way they integrate into the Godot engine.


## Table of contents
<!-- toc -->


## Terminology

To avoid confusion, whenever we talk about objects, we mean _instances of Godot classes_. This amounts to `Object` (the hierarchy's root)
and all classes inheriting directly or indirectly from it: `Node`, `Resource`, `RefCounted`, etc.

In particular, the term "class" also includes user-provided types that are declared using `#[derive(GodotClass)]`,
even if Rust technically calls them structs. In the same vein, _inheritance_ refers to the conceptual relation
("`Player` inherits `Sprite2D`"), not any technical language implementation.

Objects do **not** include built-in types such as `Vector2`, `Color`, `Transform3D`, `Array`, `Dictionary`, `Variant` etc.
These types, although sometimes called "built-in classes", are not real classes, and we generally do not refer to their instances as _objects_.


### Inheritance

Inheritance is a central concept in Godot. You likely know it already from the node hierarchy, where derived classes add specific functionality.
This concept extends to Rust classes, with inheritance being emulated via composition.

Each Rust class has a Godot base class.

- Typically, a base class is a node type, i.e. it (indirectly) inherits from the class `Node`. This makes it possible to attach instances
  of the class to the scene tree. Nodes are manually managed, so you need to either add them to the scene tree or free them manually.
- If not explicitly specified, the base class is `RefCounted`. This is useful to move data around, without interacting with the scene tree.
  "Data bundles" (collection of multiple fields without much logic) should generally use `RefCounted`.
- `Object` is the root of the inheritance tree. It is rarely used directly, but it is the base class of `Node` and `RefCounted`.
  Use it only when you really need it; it requires manual memory management and is harder to handle.

```admonish note title="Inheriting custom base classes"
You cannot inherit other Rust classes or user-defined classes declared in GDScript.

To create relations between Rust classes, use composition and traits. The library still undergoes [some exploration in this area][issue-traits],
so best practices for absracting over Rust classes might change in the future.
```


## The `Gd` smart pointer

[`Gd<T>`][api-gd] is the type you will encounter the most when working with gdext.  

It is also the most powerful and versatile type that the library provides.

In particular, its responsibilities include:

- Holding references to _all_ Godot objects, whether they are engine types like `Node2D` or your own `#[derive(GodotClass)]` structs in Rust.
- Tracking memory management of types that are reference-counted.
- Safe access to user-defined Rust objects through interior mutability.
- Detecting destroyed objects and preventing UB (double-free, dangling pointer, etc.).
- Providing FFI conversions between Rust and engine representations, for engine-provided and user-exposed APIs.

A few practical examples (don't worry if you don't fully understand them yet, they will be explained later on):

1. Retrieve a node relative to current -- type inferred as `Gd<Node3D>`:
    ```rust
    // retrieve Gd<Node3D>.
    let child = self.base().get_node_as::<Node3D>("Child");
    ```

2. Load a scene and instantiate it as a `RigidBody2D`:
    ```rust
    // mob_scene is declared as a field of type Gd<PackedScene>.
    self.mob_scene = load("res://Mob.tscn");
    // instanced is of type Gd<RigidBody2D>.
    let mut instanced = self.mob_scene.instantiate_as::<RigidBody2D>();
    ```

3. A signal handler for the `body_entered` signal of a `Node3D` in your custom class:
    ```rust
    #[godot_api]
    impl Player {
        #[func]
        fn on_body_entered(&mut self, body: Gd<Node3D>) {
            // Body holds the reference to the Node3D object that triggered the signal.
        }
    }
    ```


## Object management and lifetime

When working with Godot objects, it is important to understand how long they live and how or when they are destroyed.


### Construction

Not all classes in Godot are constructible; for example, singletons do not provide a constructor.

For all others, the constructor's name depends on the memory management of the class:

- For reference-counted classes, the constructor is called `new_gd` (e.g. `TcpServer::new_gd()`)
- For manually managed classes, it is called `new_alloc` (e.g. `Node2D::new_alloc()`).

The [`new_gd()`][api-newgd] and [`new_alloc()`][api-newalloc] functions are imported via extension traits `NewGd` and `NewAlloc`, respectively.
Those always return the type `Gd<Self>`. If you type `::` after a class name, your IDE should suggest the correct constructor for it.


### Instance API

Once alive, Godot objects can be accessed to interact with the engine.

Functionality to query and manage the object's lifetime is directly available on the `Gd<T>` type. Examples include:

- `instance_id()` to obtain Godot's object ID.
- `clone()` to create a new reference to the same object.
- `free()` to manually destroy objects.
- `==` and `!=` to compare objects for identity.


### Conversions

You can up- and downcast objects if they stand in an inheritance relation. gdext will statically ensure that the cast makes sense.

Downcasts are done via `cast::<U>()`. If the cast fails, the method will panic. You can also use `try_cast::<U>()` to get a `Result`.

```rust
let node: Gd<Node> = ...;

// "I know this downcast will succeed" -> use cast().
let node2d = node.cast::<Node2D>();
// Alternative syntax:
let node2d: Gd<Node2D> = node.cast();

// Fallible downcast -> use try_cast().
let sprite = node.try_cast::<Sprite2D>();
match sprite {
    Ok(sprite) => { /* access converted Gd<Sprite2D> */ },
    Err(node) => { /* access previous Gd<Node> */ },
}
```

Upcasts are always infallible. You can use `upcast::<U>()` to consume the value.

```rust
let node2d: Gd<Node2D> = ...;
let node = node2d.upcast::<Node>();
// or, equivalent:
let node: Gd<Node> = node2d.upcast();
```

If you just need a reference, use `upcast_ref()` or `upcast_mut()`.

```rust
let node2d: Gd<Node2D> = ...;
let node: &Node = node2d.upcast_ref();

let mut refc: Gd<RefCounted> = ...;
let obj: &mut Object = refc.upcast_mut();
```


### Destruction

Reference-counted classes, instantiated via `new_gd()`, are automatically destroyed when the last reference goes out of scope.
This includes references that have been shared with the Godot engine (e.g. held by GDScript code).

Classes instantiated via `new_alloc()` require manual memory management. This means that you either have to explicitly call
[`Gd::free()`][api-gd-free] or let a Godot method such as `Node::queue_free()` take care of it.


```admonish tip title="Safety around the dead"
Accessing destroyed objects is a common source of bugs in Godot, and can occasionally cause undefined behavior (UB).
Not so in godot-rust! We have designed the `Gd<T>` type to be safe even in the presence of mistakes.

If you try to access a destroyed object, the Rust code will panic. There are also APIs to query for validity, although we
generally recommend to fix bugs rather than defensive programming.
```


## Conclusion

Objects are a central concept in the Rust bindings. They represent instances of Godot classes, both engine- and user-defined.
We have seen how to construct, manage and destroy them. The next chapter will go into calling Godot functions.


[issue-traits]: https://github.com/godot-rust/gdext/issues/426
[api-gd-from-init-fn]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.from_init_fn
[api-gd-free]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.free
[api-gd]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html
[api-newalloc]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.NewAlloc.html
[api-newgd]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.NewGd.html
