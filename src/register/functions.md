<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering functions

Functions are essential in any programming language to execute logic. The gdext library allows you to register functions, so that they can
be called from the Godot engine and GDScript.

Registration of functions happens always inside `impl` blocks that are annotated with `#[godot_api]`.

See also [GDScript reference for functions][godot-gdscript-functions].


## Table of contents

<!-- toc -->


## Godot special functions

The functions provided by the interface trait (beginning with `I`) are called _Godot special functions_.

You already saw a function being registered: the `init` constructor, which takes a base object and returns an instance of `Self`.
Godot additionally provides several functions for you to override, to hook into the lifecycle of your object.

Here is a small selection of lifecycle methods. For a complete list, see [`INode3D` docs][api-inode3d].

```rs
#[godot_api]
impl INode3D for Monster {
    // Instantiate the object.
    fn init(base: Base<Node3D>) -> Self { ... }
    
    // Called when the node is ready in the scene tree.
    fn ready(&mut self) { ... }
    
    // Called every frame.
    fn process(&mut self, delta: f64) { ... }
    
    // Called every physics frame.
    fn physics_process(&mut self, delta: f64) { ... }
    
    // String representation of the object.
    fn to_string(&self) -> GString { ... }
    
    // Handle user input.
    fn input(&mut self, event: Gd<InputEvent>) { ... }
    
    // Handle lifecycle notifications.
    fn on_notification(&mut self, what: Node3DNotification) { ... }
}
```

As you see, some methods take `&mut self` and some take `&self`, depending on whether they typically mutate the object or not. Some also have
return values, which are passed back into the engine. For example, the `GString` returned from `to_string()` is used if you print an object
in GDScript.

So let's implement `to_string()`, here again showing the class definition for quick reference.

```rs
#[derive(GodotClass)]
#[class(base=Node3D)]
struct Monster {
    name: String,
    hitpoints: i32,
    
    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Monster {   
    ... // init etc.
    
    fn to_string(&self) -> GString {
        let Self { name, hitpoints, .. } = &self;
        format!("Monster(name={name}, hp={hitpoints})").into()
    }
}
```


## User-defined functions


### Methods

Besides Godot special functions, you can register your own functions. You need to declare them inside an inherent `impl` block, also annotated
with `#[godot_api]`.

Each function needs a `#[func]` attribute to register it with Godot. You can omit `#[func]` as well, but functions defined like that are only
visible to Rust code.

Let's add two methods to our `Monster` class: one that deals damage to the monster, and one that returns its name.

```rs
#[godot_api]
impl Monster {
    #[func]
    fn damage(&mut self, amount: i32) {
        self.hitpoints -= amount;
    }
    
    #[func]
    fn get_name(&self) -> GString {
        self.name.clone()
    }
}
```

The above methods are now available in GDScript. You can call them as follows:

```php
var monster = Monster.new()
# ...
monster.damage(10)
print("A monster called ", monster.get_name())
```

As you see, the Rust types are automatically mapped to their GDScript counterparts. In this case, `i32` becomes `int` and `GString` becomes
`String`. Sometimes there are multiple possible mappings, e.g. Rust `u16` would also be mapped to `int` in GDScript.


### Associated functions and constructors

In addition to **methods** (taking `&self` or `&mut self`), you can also register **associated functions** (without a receiver). In GDScript,
the latter are known as "static functions".

Associated functions are often useful for user-defined constructors.

Previously, we talked about the default constructor `init` not being very useful, as it leaves the `Monster` in an incorrect state.
Let's provide a more suitable constructor, which accepts name and hitpoints as parameters.

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
have a _value_ of the raw object. You never need to return `Self` in your own defined functions (unless it's pure Rust code).

For details, consult [the chapter about objects][book-objects] or the [`Gd<T>` API docs][api-gd].
```

So we need to return `Gd<Self>` instead of `Self`.

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

<!-- TODO: base() + base_mut() -->
<!-- TODO: bind() + bind_mut() and their relation to &self/&mut self -->


## Conclusion

This page gave you an overview of registering functions with Godot:

- Special methods that hook into the lifecycle of your object.
- User-defined methods that query or modify the state of your instance.
- Associated functions that serve as constructors.

These are just a few use cases, you are very flexible in how you design your interface between Rust and GDScript.
In the next page, we will look into properties.

[godot-gdscript-functions]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#functions
[api-inode3d]: https://godot-rust.github.io/docs/gdext/master/godot/engine/trait.INode3D.html
[api-gd]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html
[book-objects]: ../intro/objects.md
[api-gd-from-init-fn]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.from_init_fn
