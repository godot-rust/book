<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Script-virtual functions

The GDExtension API allows you to define virtual functions in Rust, which can be overridden in scripts attached to your objects.

Note that these are conceptually different from virtual functions like `ready()`, which are defined _by Godot_ and overridden _by you_ (in Rust).
Hence the emphasis on "script-virtual".

```admonish note title="Compatibility"
This feature is available from Godot 4.3 onwards.  
```


## Table of contents

<!-- toc -->


## A motivating example

To stay with our `Monster` example, let's say we have different monster types and would like to customize their behavior. We can write the logic
common to all monsters in Rust, and for quick prototyping use GDScript for the specific parts.

For example, we can experiment with two monsters: `Orc` and `Goblin`. Each of them comes with a different behavior, which is encoded in
a respective GDScript file. The project structure might look like this:

```txt
project_dir/
│
├── godot/
│   ├── .godot/
│   ├── project.godot
│   ├── MonsterGame.gdextension
│   └── Scenes
│       ├── Monster.tscn
│       ├── Orc.gd
│       └── Goblin.gd
│
└── rust/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── monster.rs
```

The `Monster.tscn` encodes a simple scene with the node `Monster` (our Rust class inheriting `Node3D`) at the root. This node would be the
one to attach scripts to.


## Step by step


### Rust default behavior

Let's start from this class definition:

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Monster {
    base: Base<Node3D>
}
```

We can now implement a Rust function to calculate the damage a monster deals per hit. Traditionally, we would write this:

```rust
#[godot_api]
impl Monster {
    #[func]
    fn damage(&self) -> i32 {
        10
    }
}
```

That method will always return `10`, no matter what. To customize this behavior in scripts that are attached to the `Monster` node, we can
define a _virtual method_ in Rust, which can be _overridden_ in GDScript. The Rust code is called the _default_ implementation.

```admonish note title="Early vs. late binding"
_Virtual_ (also called _late-binding_) means that dynamic dispatch is involved: the actual method to call is determined at runtime, 
depending on whether a script is attached to the `Monster` node -- and if yes, which one.

This stands in contrast to _early-binding_, which is resolved at compile time, using static dispatch.
```

While traditional Rust might use trait objects (`dyn Trait`) for late binding, godot-rust provides a more direct way.
Making a method virtual is very easy: just add the `virtual` key to the `#[func]` attribute.

```rust
#[godot_api]
impl Monster {
    #[func(virtual)]
    fn damage(&self) -> i32 {
        10
    }
}
```

That's it. Your monster can now be customized in scripts.


### Overriding in GDScript

In the GDScript files, you can now override the Rust `damage` method as `_damage`. The method is prefixed with an underscore, following Godot
convention for virtual methods such as `_ready` or `_process`.

Here's an example for the `Orc` and `Goblin` scripts:

```php
# Orc.gd
extends Monster

func _damage():
    return 20
```

```php
# Goblin.gd
extends Monster

# Random damage between 5 and 15.
# Type annotations are possible, but not required.
func _damage() -> int:
    return randi() % 11 + 5
```

If your signature in GDScript does not match the Rust signature, Godot will cause an error.


### Dynamic behavior

Now, let's call `damage()` in Rust code:

```rust
fn monster_attacks_player(monster: Gd<Monster>, player: Gd<Player>) {
    // Compute the damage.
    let damage_points: i32 = monster.bind().damage();

    // Apply the damage to the player.
    player.bind_mut().take_damage(damage_points);
}
```

What value does `damage_points` have in the above example?  
The answer depends on the circumstances:

- If the `Monster` node has no script attached, `damage_points` will be `10` (the default implementation in Rust).
- If the `Monster` node has the `Orc.gd` script attached, `damage_points` will be `20`.
- If the `Monster` node has the `Goblin.gd` script attached, `damage_points` will be a random number between `5` and `15`.


## Trade-offs

You might ask: what's the point of all this, if one can achieve the same with a simple `match` statement?

And you're right; if a `match` in Rust is all you need, then use that. However, the script-based approach has a few advantages, especially
when it comes to more complex scenarios than just computing a single damage number:

- You can prepare a variety of scripts with different behaviors, e.g. for different levels or enemy AI behavior. In the Godot editor, you
  can then simply swap out scripts as needed, or have different `Monster` instances with different scripts, to compare them side-by-side.
- Switching behaviors does not require recompiling Rust code. This can be useful if you work with game designers, modders or artists who
  are less familiar with Rust, but want to experiment nonetheless.

That said, if your compile times are short (gdext itself is quite lightweight) and you prefer having the logic in Rust, that is of course
also a valid choice. To retain the option to quickly switch behaviors, you could use an `#[export]`'ed enum to select the behavior, and
then dispatch on that in Rust.

Ultimately, `#[func(virtual)]` is just one extra tool that godot-rust offers among a variety of abstraction mechanisms. Since Godot's
paradigm revolves heavily around attaching scripts to nodes, this feature integrates very well with the engine.


## Limitations

```admonish warning title="Warning"
Godot script-virtual functions do not behave like OOP virtual functions in every aspect.  
Make sure you understand the limitations.
```


In contrast to virtual methods from OOP languages (C++, C#, Java, Kotlin, PHP, ...), there are some important differences to be aware of.

1. **The default implementation is unreachable from Godot.**

   In Rust, calling `monster.bind().damage()` will automatically look for script overrides, and fall back to the Rust default if no script is
   attached. In GDScript however, you cannot call the default implementation. Calling `monster._damage()` will fail without a script.
   The same is true for reflection calls from Rust (e.g. `Object::call()`).

   The `_` prefix underlines that: ideally, you don't call virtual functions directly from scripts.

   To work around this, you can declare a separate `#[func] fn default_damage()` in Rust, which will be registered as a regular method and
   thus can be called from scripts. To keep Rust's convenient fallback behavior, just invoke `default_damage()` inside the Rust `damage()` method.

2. **No access to `super` methods.**

   In OOP languages, you can call the base method from the overriding method, typically using `super` or `base` keywords.

   As a consequence of point 1), this default method is also not visible to the script overriding it. The same workaround can be used though.

3. **Limited re-entrancy.**

    If you call a virtual method from Rust, it may dispatch to a script implementation. The Rust side holds either a shared (`&self`) or
    exclusive borrow (`&mut self`) to the object -- an implicit `Gd::bind()` or `Gd::bind_mut()` guard. If the script implementation then
    accesses the same object (e.g. by setting a `#[var]` property), panics can occur due to double-borrow errors.

    For now, you can work around this by declaring the method with `#[func(gd_self, virtual)]`. The `gd_self` requires the first
    parameter to be of type `Gd<Self>`, which avoids the bind call and thus the borrow.

We are observing how virtual functions are used by the community and plan to mitigate the limitations where possible. If you have any inputs,
feel free to let us know!


## Types of scripts

While this page focuses on GDScript, Godot also provides other scripting capabilities. Notably, [C# can be used for scripting][godot-csharp], if
you run Godot with the Mono runtime.

The library also provides a dedicated trait [`ScriptInstance`][api-scriptinstance], which allows users to provide Rust-based "scripts".
Consult its docs for detailed information.

You can also configure scripts entirely programmatically, using the [`classes::Script`][api-class-script] API and its inherited classes, such
as [`classes::GDScript`][api-class-gdscript]. This typically defeats the purpose of scripting, but is mentioned here for completeness.


## Conclusion

In this chapter, we have seen how to define virtual functions in Rust, and how to override them in GDScript. This provides an additional
integration layer between the two languages and allows to effortlessly experiment with swappable behaviors from the editor.

[api-class-gdscript]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.GDScript.html
[api-class-script]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Script.html
[api-scriptinstance]: https://godot-rust.github.io/docs/gdext/master/godot/obj/script/trait.ScriptInstance.html
[godot-csharp]: https://docs.godotengine.org/en/stable/tutorials/scripting/c_sharp/index.html
