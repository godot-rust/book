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

```java
signal damage_taken
signal damage_taken(amount)
signal damage_taken(amount: int)
```

However, the difference between the above declarations is purely informational (e.g. appears in class docs).
Let's look at an example:

```java
signal damage_taken(amount: int)

func log_damage():
	print("damaged!")

func _ready():
	damage_taken.connect(log_damage)
```

Note how `log_damage()` has no parameters, yet you can connect it without warning, neither at parse time nor at runtime.

This problem isn't limited to `connect()`; let's pass an argument of wrong type to `emit()`:

```php
signal damage_taken(amount: int)

func log_damage(amount): # now with parameter
	print("damaged: ", amount)

func _ready():
	damage_taken.connect(log_damage)
	damage_taken.emit(true) # no int, no worries -> prints "damaged: true"
```

Again, GDScript happily passes through `bool`, despite the signal declaring `int`.

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
    base: Base<Node3D>, // required when declaring signals.
}

#[godot_api]
impl Monster {
    #[signal]
    fn damage_taken(amount: i32);
}
```

Signal syntax is close to `#[func]`, but it needs a semicolon instead of a function body. Receivers (`&self`, `&mut self`) and return types
are not supported.

### Generated code

As soon as you register at least one signal, godot-rust will implement the [`WithSignals`][api-withsignals] trait for your class.
This provides the `signals()` method, which can now be accessed inside class methods. `signals()` returns a "signal collection", i.e.
a struct which lists all signals as methods.

```rust
impl INode3D for Monster {
    fn ready(&mut self) {
        let sig = self.signals().damage_taken();
    }
}
```

The `damage_taken()` method returns a custom-generated struct, with an API tailored to the signature of `fn damage_taken(amount: i32)`. 

That struct is implementation-defined. Besides the customized API, it also implements `Deref/DerefMut` to [`TypedSignal`][api-typedsignal],
which exposes further APIs to interact with the signal.

## Connecting signals

It is quite common to connect signals to methods of the same class. This is possible with the `connect_self()` method. As an argument, you
simply pass the method pointer:

```rust
impl INode3D for Monster {
    fn ready(&mut self) {
        self.signals()
            .damage_taken()
            .connect_self(Self::on_damage_taken);
    }
}

impl Monster {
    fn on_damage_taken(&mut self, amount: i32) {
        // Update healthbar, play sound, etc.
    }
}
```

Note how `on_damage_taken` has no `#[func]` attribute, and its surrounding impl block no `#[godot_api]` proc-macro. Signal receivers are
regular Rust functions! You can completely hide them from Godot, and only make them accessible via signals.

Also, the functions



[api-object]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html
[api-signal]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#signals
[api-withsignals]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.WithSignals.html
[godot-gdscript-signals]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#signals
[api-typedsignal]: https://godot-rust.github.io/docs/gdext/master/godot/register/struct.TypedSignal.html
