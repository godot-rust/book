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

```admonish info title="Interface traits"
Each engine class comes with an associated trait, which has the same name but is prefixed with the letter `I`, for "Interface".
The trait has no required functions, but you can override any functions to customize the behavior towards Godot.

Any `impl` block for the trait must be annotated with the `#[godot_api]` attribute macro.
```

```admonish info title="godot_api macro"
The attribute proc-macro `#[godot_api]` is applied to `impl` blocks and marks their items for registration.
It takes no arguments.

See [API docs][api-godot-api] for detailed information.
```

Functions provided by the interface trait (beginning with `I`) are called _Godot special functions_. These can be overridden and allow you
to influence the behavior of an object. Most common is a hook into the _lifecycle_ of your object, defining some logic that is run upon
certain events like creation, scene-tree entering, or per-frame updates.

In our case, the `Node3D` comes with the `INode3D` trait.
Here is a small selection of its lifecycle methods. For a complete list, see [`INode3D` docs][api-inode3d].

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
    hitpoints: i3l1
    
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Monster {      
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


### Associated functions

In addition to **methods** (taking `&self` or `&mut self`), you can also register **associated functions** (without a receiver). In GDScript,
the latter are known as "static functions".

For example, we can add an associated function which generates a random monster name:

```rs
#[godot_api]
impl Monster {
    #[func]
    fn random_name() -> GString {
        // ...
    }
}
```

The above can then be called from GDScript as follows:

```php
var name: String = Monster.random_name()
```

Of course, it is also possible to declare parameters.

Associated functions are sometimes useful for user-defined constructors, as we will see in the next chapter.

<!-- TODO: base() + base_mut() -->
<!-- TODO: bind() + bind_mut() and their relation to &self/&mut self -->


## Conclusion

This page gave you an overview of registering functions with Godot:

- Special methods that hook into the lifecycle of your object.
- User-defined methods and associated functions to expose a Rust API to Godot.

These are just a few use cases, you are very flexible in how you design your interface between Rust and GDScript.
In the next page, we will look into a special kind of functions: constructors.

[api-godot-api]: https://godot-rust.github.io/docs/gdext/master/godot/register/attr.godot_api.html
[api-inode3d]: https://godot-rust.github.io/docs/gdext/master/godot/engine/trait.INode3D.html
[godot-gdscript-functions]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#functions
