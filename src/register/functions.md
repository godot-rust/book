<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering functions

Functions are essential in any programming language to execute logic. The godot-rust library allows you to register functions, so that they can
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

```rust
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

```rust
#[derive(GodotClass)]
#[class(base=Node3D)]
struct Monster {
    name: String,
    hitpoints: i32,
    
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

```rust
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

```rust
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


## Methods and object access

When you define your own Rust functions, there are two use cases that occur very frequently:

- You want to invoke your Rust methods from outside, through a `Gd` pointer.
- You want to access methods of the base class (e.g. `Node3D`).

This section explains how to do both.


### Calling Rust methods (binds)

If you now have a `monster: Gd<Monster>`, which stores a `Monster` object as defined above, you won't be able to simply call
`monster.damage(123)`. Rust is stricter than C++ and requires that only one `&mut Monster` reference exists at any point in time. Since
`Gd` pointers can be freely cloned, direct access through `DerefMut` wouldn't be sufficient to ensure non-aliasing.

To approach this, godot-rust uses the interior mutability pattern, which is quite similar to how [`RefCell`][rust-refcell] works.

In short, whenever you need shared (immutable) access to a Rust object from a `Gd` pointer, use [`Gd::bind()`][api-gd-bind].
Whenever you need exclusive (mutable) access, use [`Gd::bind_mut()`][api-gd-bindmut].

```rust
let monster: Gd<Monster> = ...;

// Immutable access with bind():
let name: GString = monster.bind().get_name();

// Mutable access with bind_mut() -- we rebind the object first:
let mut monster = monster;
monster.bind_mut().damage(123);
```

Regular Rust visibility rules apply: if your function should be visible in another module, declare it as `pub` or `pub(crate)`.

```admonish note title="The need for #[func]"
The `#[func]` attribute _only_ makes a function available to the Godot engine. It is orthogonal to Rust visibility (`pub`, `pub(crate)`, ...)
and does not influence whether a method can be accessed through `Gd::bind()` and `Gd::bind_mut()`.

If you only need to call a function in Rust, do not annotate it with `#[func]`. You can always add this later.
```

`bind()` and `bind_mut()` return _guard objects_. At runtime, the library verifies that the borrow rules are upheld, and panics otherwise.
It can be beneficial to reuse guards across multiple statements, but make sure to keep their scope limited to not unnecessarily constrain access
to objects (especially when using `bind_mut()`).

```rust
fn apply_monster_damage(mut monster: Gd<Monster>, raw_damage: i32) {
    // Artificial scope:
    {
        let guard = monster.bind_mut(); // locks object -->
        let armor = guard.get_armor_multiplier();
        
        let damage = (raw_damage as f32 * armor) as i32;

        guard.damage(damage)
    } // <-- until here, where guard lifetime ends.

    // Now you can pass the pointer on to other routines again.
    check_if_dead(monster);
}
```


### Base access from `self`

Within a class, you don't directly have a `Gd<T>` pointing to the own instance with base class methods. So you cannot use the approach explained
in the [_Calling functions_ chapter][book-godot-api-functions], where you would simply use `gd.set_position(...)` or similar.

Instead, you can access base class APIs via [`base()` and `base_mut()`][api-withbasefield-base]. This requires that your class defines a
`Base<T>` field. Let's say we add a `velocity` field and two new methods:

```rust
#[derive(GodotClass)]
#[class(base=Node3D)]
struct Monster {
    // ...
    velocity: Vector2,
    base: Base<Node3D>,
}

#[godot_api]
impl Monster {
    pub fn apply_movement(&mut self, delta: f32) {
        // Read access:
        let pos = self.base().get_position();
      
        // Write access (mutating methods):
        self.base_mut().set_position(pos + self.velocity * delta)
    }

    // This method has only read access (&self).
    pub fn is_inside_area(&self, rect: Rect2) -> String 
    {
        // We can only call base() here, not base_mut().
        let node_name = self.base().get_name();
        
        format!("Monster(name={}, velocity={})", node_name, self.velocity)
    }
}
```

Both `base()` and `base_mut()` are defined in an extension trait [`WithBaseField`][api-withbasefield]. They return _guard objects_, which prevent
other access to `self` in line with Rust's borrow rules. You can reuse a guard across multiple statements, but make sure to keep its scope
limited to not unnecessarily constrain access to `self`:

```rust
    pub fn apply_movement(&mut self, delta: f32) {
        // Artificial scope:
        {
            let guard = self.base_mut(); // locks `self` -->
            let pos = guard.get_position();
  
            guard.set_position(pos + self.velocity * delta)
        } // <-- until here, where guard lifetime ends.
  
        // Now can invoke other self methods again.
        self.on_position_updated();
    }
```


Instead of an extra scope, you can of course also just call [`drop(guard)`][rust-mem-drop].


```admonish note title="Do not combine bind/bind_mut + base/base_mut"
Code like `object.bind().base().some_method()` is unnecessarily verbose and slow.  
If you have a `Gd<T>` pointer, use `object.some_method()` directly. 

Combining `bind()`/`bind_mut()` immediately with `base()`/`base_mut()`
is a mistake. The latter two should only be called from within the class `impl`. 
```


### Obtaining `Gd<Self>` from within

In some cases, you need to get a `Gd<T>` pointer to the current instance. This can occur if you want to pass it to other methods, or if you need
to store a pointer to `self` in a data structure.

`WithBaseField` offers a method `to_gd()`, returning a `Gd<Self>` with the correct type.

Here’s an example. The `monster` is passed a hash map, in which it can register/unregister itself, depending on whether it's alive or not.

```rust
#[godot_api]
impl Monster {
    // Function that registers each monster by name, or unregisters it if dead.
    fn update_registry(&self, registry: &mut HashMap<String, Gd<Monster>>) {
        if self.is_alive() {
            let self_as_gd: Gd<Self> = self.to_gd();
            registry.insert(self.name.clone(), self_as_gd);
        } else {
            registry.remove(&self.name);
        }
    }
}
```

```admonish warning title="Don't bind to_gd() inside class methods"
The methods `base()` and `base_mut()` use a clever mechanism that "re-borrows" the current object reference. This enables re-entrant calls,
such as `self.base().notify(...)`, which may e.g. call `ready(&mut self)`. The `&mut self` here is a reborrow of the call-site `self`.

When you use `to_gd()`, the borrow checker will treat this as an independent object. If you call `bind_mut()` on it, while inside the class impl,
you will immediately get a double-borrow panic. Intead, use `to_gd()` to hand out a pointer and don't access until the current method has ended.
```


## Conclusion

This page gave you an overview of registering functions with Godot:

- Special methods that hook into the lifecycle of your object.
- User-defined methods and associated functions to expose a Rust API to Godot.

It also showed how methods and objects interact: calling Rust methods through `Gd<T>` and working with base class APIs.

These are just a few use cases, you are very flexible in how you design your interface between Rust and GDScript.
In the next page, we will look into a special kind of functions: constructors.

[api-godot-api]: https://godot-rust.github.io/docs/gdext/master/godot/register/attr.godot_api.html
[api-inode3d]: https://godot-rust.github.io/docs/gdext/master/godot/classes/trait.INode3D.html
[godot-gdscript-functions]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#functions
[api-withbasefield]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.WithBaseField.html
[api-withbasefield-base]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.WithBaseField.html#method.base
[rust-refcell]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[rust-mem-drop]: https://doc.rust-lang.org/std/mem/fn.drop.html
[book-godot-api-functions]: ../godot-api/functions.html#godot-functions
[api-gd-bind]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/struct.Gd.html#method.bind
[api-gd-bindmut]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/struct.Gd.html#method.bind_mut
