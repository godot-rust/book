<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Constructors

While Rust does not have constructors as a language feature (like C++ or C#), associated functions that return a new object are commonly
called "constructors". We extend the term to include slightly deviating signatures, but conceptually _constructors_ are always
used to construct new objects.

Godot has a special constructor, which we call the _Godot default constructor_ or simply `init`. This is comparable to the `_init` method in
GDScript.


## Table of contents

<!-- toc -->


## Default constructor

The constructor of any `GodotClass` object is called `init` in gdext. This constructor is necessary to instantiate the object in Godot.
It is invoked by the scene tree or when you write `Monster.new()` in GDScript.

There are two options to define the constructor: let gdext generate it or define it manually. It is also possible to opt out of `init` if you
don't need Godot to default-construct your object.


### Library-generated `init`

You can use `#[class(init)]` to generate a constructor for you. This is limited to simple cases, and it calls `Default::default()` for each
field (except the `Base<T>` one, which is correctly wired up with the base object).

```rs
#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Monster {
    name: String,          // initialized to ""
    hitpoints: i32,        // initialized to 0
    base: Base<Node3D>,    // wired up
}
```

To provide another default value, use `#[init(default = value)]`. This should only be used for simple cases, as it may lead to difficult-to-read
code and error messages. This API may also still change.

```rs
#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Monster {
    name: String,          // initialized to ""
   
    #[init(default = 100)]
    hitpoints: i32,        // initialized to 100
    
    base: Base<Node3D>,    // wired up
}
```


### Manually defined `init`

We can provide a manually-defined constructor by overriding the trait's associated function `init`:

```rs
#[derive(GodotClass)]
#[class(base=Node3D)] // No init here, since we define it ourselves.
struct Monster {
    name: String,
    hitpoints: i32,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Monster {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            name: "Nomster".to_string(),
            hitpoints: 100,
            base,
        }
    }
}
```

As you can see, the `init` function takes a `Base<Node3D>` as its one and only parameter. This is the base class instance, which is typically
just forwarded to its corresponding field in the struct, here `base`.

The `init` method always returns `Self`. You may notice that this is currently the only way to construct a `Monster` instance. As soon as your
struct contains a base field, you can no longer provide your own constructor, as you can't provide a value for that field. This is by design and
ensures that _if_ you need access to the base, that base comes from Godot directly.

However, fear not: you can still provide all sorts of constructors, they just need to go through dedicated functions that internally call `init`.
More on this in the next section.


### Disabled `init`

You don't always need to provide a default constructor to Godot. Reasons to not have a constructor include:

- Your class is not a node that should be added to the tree as part of a scene file.
- You require custom parameters to be provided for your object invariants -- a default value is not meaningful.
- You only need to construct objects from Rust code, not from GDScript or the Godot editor.

To disable the `init` constructor, you can use `#[class(no_init)]`:

```rs
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
struct Monster {
    name: String,
    hitpoints: i32,
    base: Base<Node3D>,
}
```

Not providing/generating an `init` method and forgetting to use `#[class(no_init)]` will result in a compile-time error.


## Custom constructors

The default constructor `init` is not always useful, as it may leave objects in an incorrect state.

For example, a `Monster` will always have the same values for `name` and `hitpoints` upon construction, which may not be desired.
Let's provide a more suitable constructor, which accepts those attributes as parameters.

```rs
// Default constructor from before.
#[godot_api]
impl INode3D for Monster {
    fn init(base: Base<Node3D>) -> Self { ... }
}

// New custom constructor.
#[godot_api]
impl Monster {
    #[func] // Note: the following is incorrect.
    fn from_name_hp(name: GString, hitpoints: i32) -> Self { 
        ...
    }
}
```

But now, how to fill in the blanks? `Self` requires a base object, how to obtain it? In fact, we cannot return `Self` here.

```admonish info title="Passing around objects"
When interacting with Godot from Rust, all objects (class instances) need to be transported inside the `Gd` smart pointer -- whether
they appear as parameters or return types.

The return types of `init` and a few other gdext-provided functions are an exception, because the library requires at this point that you
have a _value_ of the raw object. You never need to return `Self` in your own defined `#[func]` functions.

For details, consult [the chapter about objects][book-objects] or the [`Gd<T>` API docs][api-gd].
```

So we need to return `Gd<Self>` instead of `Self`.


### Objects with a base field

To construct `Gd<Self>`, we can use [`Gd::from_init_fn()`][api-gd-from-init-fn], which takes a closure. This closure accepts a `Base` object
and returns an instance of `Self`. In other words, it has the same signature as `init` -- this presents an alternative way of constructing
Godot objects, while allowing to pass in addition context.

The result of `Gd::from_init_fn()` is a `Gd<Self>` object, which can be directly returned by `Monster::from_name_hp()`.

```rs
#[godot_api]
impl Monster {
    #[func]
    fn from_name_hp(name: GString, hitpoints: i32) -> Gd<Self> {
        // Function contains a single statement, the `Gd::from_init_fn()` call.
        
        Gd::from_init_fn(|base| {
            // Accept a base of type Base<Node3D> and directly forward it.
            Self {
                name: name.into(), // Convert GString -> String.
                hitpoints,
                base,
            }
        })
    }
}
```

That's it! The just added associated function is now registered in GDScript and effectively works as a constructor:

```php
var monster = Monster.from_name_hp("Nomster", 100)
```


### Objects without a base field

For classes that don't have a base field, you can simply use [`Gd::from_object()`][api-gd-from-object] instead of `Gd::from_init_fn()`.

This is often useful for _data bundles_, which don't define much logic but are an object-oriented way to bundle related data in a single
type. Such classes are typically subclasses of `RefCounted` or `Resource`.

```rs
#[derive(GodotClass)]
#[class(no_init)] // We only provide a custom constructor.
// Since there is no #[class(base)] key, the base class will default to RefCounted.
struct MonsterConfig {
    color: Color,
    max_hp: i32,
    tex_coords: Vector2i,
}

#[godot_api]
impl MonsterConfig {
    // Not named 'new' since MonsterConfig.new() in GDScript refers to default. 
    #[func] 
    fn create(color: Color, max_hp: i32, tex_coords: Vector2i) -> Gd<Self> {
        Gd::from_object(Self {
            color,
            max_hp,
            tex_coords,
        })
    }
}
```


## Conclusion

Constructors allow to initialize Rust classes in various ways. You can generate, implement, or disable the default constructor `init`, and you
can provide as many custom constructors with different signatures as you like.

[api-gd-from-init-fn]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.from_init_fn
[api-gd-from-object]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.from_object
[api-gd]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html
[book-objects]: ../intro/objects.md
