<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering signals

Signals are a Godot mechanism that implement the Observer pattern. They allow you to emit events, which are received by everyone who is 
subscribed ("connected") to the signal, decoupling sender and receiver. If you haven't worked with Godot signals before, you should definitely
read the [GDScript reference for signals][godot-gdscript-signals].

## The problem with GDScript signals

You can define GDScript signals as follows, with optional parameter names and types:

```php
signal damage_taken
signal damage_taken(amount)
signal damage_taken(amount: int)
```

However, as of Godot 4.4, the difference between the above declarations is purely informational (e.g. appears in class docs).
Let's look at an example:

```php
signal damage_taken(amount: int)

func log_damage():
	print("damaged!")

func _ready():
	damage_taken.connect(log_damage)
```

Note how `log_damage()` has no parameters, yet you can connect it without warning, neither at parse time nor at runtime.

Now let's check what happens when we pass a different type to `emit()`:

```php
signal damage_taken(amount: int)

func log_damage(amount): # now with parameter
	print("damaged: ", amount)

func _ready():
	damage_taken.connect(log_damage)
	damage_taken.emit(true) # no int, no worries -> prints "damaged: true"
```

Again, GDScript happily accepts `bool` being passed.

```admonish warning "GDScript signals are not type-safe"
In GDScript, a `signal` parameter list is **not type-checked**.
 
Mismatching `connect()` or `emit()` calls may or may not be caught at runtime, based on the handler function's own typing.

Passing around functions requires `Callable`, which is also untyped.
```

While it seems like a minor issue in examples like the above, this becomes hard to track in bigger projects with many similar signals, especially
once you start refactoring. A signal acts as an API between the sender and receiver, but there is no way to verify this contract, besides a high
level of manual discipline and testing.

## Rust signals

godot-rust provides a fully type-safe and ergonomic API to connect and emit signals, even though they are untyped in GDScript.
You can rely on signatures and don't need to fear refactorings, as Rust will catch any mismatches at compile time.

In Rust, signals can be defined with the `#[signal]` attribute inside a `#[godot_api]` block.
Let's take again our class from earlier:

```rust
#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Monster {
    hitpoints: i32,
    base: Base<Node3D>, // now required.
}

#[godot_api]
impl Monster {
    #[signal]
    fn damage_taken(amount: i32);
}
```

Their syntax is close to `#[func]`, but they have a semicolon instead of a function body, no `&self`/`&mut self` receiver and no return
type.





[api-object]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html
[api-signal]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#signals
[godot-gdscript-signals]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#signals
