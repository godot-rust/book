<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering signals

Signals are a Godot mechanism to implement the Observer pattern. You can emit events, which are received by everyone who is subscribed
("connected") to the signal, decoupling sender and receiver. If you haven't worked with Godot signals before, you should definitely
read the [GDScript tutorial][godot-gdscript-signals].


## Table of contents

<!-- toc -->


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

```admonish danger title="GDScript signals are not type-safe"
In GDScript, a `signal` parameter list is **not type-checked**.
 
Mismatching `connect()` or `emit()` calls may or may not be caught at runtime, based on the handler function's own typing.
They are never caught at parse time.
```

While this seems like a minor issue in examples like the above, this becomes hard to track in bigger projects with many similar signals,
especially once you start refactoring. A signal is designed to act as an API between the sender and receiver -- but there is no way to verify
this interface contract, apart from a high level of manual discipline and testing.


## Rust signals

godot-rust provides a type-safe and straightforward API to connect and emit signals, even though they are untyped in GDScript.
You can rely on signatures and don't need to fear refactorings, as Rust will catch any mismatches at compile time.

In Rust, signals can be defined with the `#[signal]` attribute inside a `#[godot_api]` block.
Let's take again our class from earlier and declare a `damage_taken` signal:

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
This provides the `signals()` method, which can now be accessed inside class methods.

`signals()` returns a _signal collection_, i.e. a struct which exposes all signals as named methods:

```rust
// Generated code ($ are placeholders, actual names up to implementation):
impl $SignalCollection {
    fn damage_taken(&mut self) -> $Signal {...}
}

#[godot_api]
impl INode3D for Monster {
    fn ready(&mut self) {
        let sig = self.signals().damage_taken();
    }
}
```

The `damage_taken()` method returns a custom-generated _signal type_ (referred to as `$Signal` in the snippet), whose API is tailored to the
signature of `fn damage_taken(amount: i32)`. Each `#[signal]` attribute generates a distinct signal type.

The signal type is implementation-defined. Besides the `#[signal]`-specific custom API, it also implements `Deref/DerefMut` with target
[`TypedSignal`][api-typedsignal], meaning you can additionally use all _those_ methods on each signal type.


## Connecting signals

godot-rust offers many ways to connect signals, depending on where the handler function is located.


### Signal + handler on same object `self`

Connecting signals to methods of the same class is quite common. This is possible with the `connect_self()` method, which simply takes the
method pointer as an argument:

```rust
impl Monster {
    fn on_damage_taken(&mut self, amount: i32) {
        ... // Update healthbar, play sound, etc.
    }
}

#[godot_api]
impl INode3D for Monster {
    fn ready(&mut self) {
        self.signals()
            .damage_taken()
            .connect_self(Self::on_damage_taken);
    }
}
```

Note how `on_damage_taken` has no `#[func]` attribute, and its surrounding impl block no `#[godot_api]` proc-macro. Signal receivers are
regular Rust functions! You can completely hide them from Godot, and only make them accessible via signals.

Since `connect_self()`'s parameter here is essentially `impl FnMut(&mut Self, i32)`, you can also pass a closure:

```rust
#[godot_api]
impl INode3D for Monster {
    fn ready(&mut self) {
        self.signals()
            .damage_taken()
            .connect_self(|this: &mut Self, amount| {
                //               ^^^^^^^^^
                //               must be explicit; other parameters types are inferred.

                ... // Update healthbar, play sound, etc.
            });
    }
}
```


### Handler on different object

If the handler function should run on an object other than `self`, you can use `connect_obj()`, which takes a `&Gd<T>` as first argument:

```rust
#[godot_api]
impl INode3D for Monster {
    fn ready(&mut self) {
        // Let's say damage is deflected to a shield object.
        // That one is stored as field `shield: OnReady<Gd<Shield>>`.
        // &*self.shield is thus `&Gd<Shield>` we need.
        
        self.signals()
            .damage_taken()
            .connect_obj(&*self.shield, Shield::on_damage_taken);
    }
}
```


### Handler without object (associated/static function)

If the handler function does not need access to `self`, simply use `connect()`:

```rust
impl Monster {
    // Now an associated function, no longer a method.
    fn on_damage_taken(amount: i32) {
        // Does not modify the object itself, but updates
        // some global statistics.
    }
}

#[godot_api]
impl INode3D for Monster {
    fn ready(&mut self) {
        self.signals()
            .damage_taken()
            .connect(Self::on_damage_taken);
        
        // Or with closures:
        self.signals()
            .damage_taken()
            .connect(|amount| {
                // Update global statistics.
            });
    }
}
```


## Emitting signals

We already saw that `#[signal]` attributes generate a signal type with several methods: `connect()`, `connect_self()` and `connect_obj()`.
This same signal type also provides an `emit()` method, which you can use to trigger the signal:

```rust
impl Monster {
    // Can be invoked by other game systems.
    pub fn deal_damage(&mut self, amount: i32) {
        self.hitpoints -= amount;
        self.signals().damage_taken().emit(amount);
    }
}
```

Like `connect*()` methods, `emit()` is fully type-safe. You can only pass a single `i32`. If you update your signal definition, e.g. to take a
`bool` or `enum` value for the type of damage, the compiler will catch all `connect*` and `emit` calls. You'll sleep well after refactorings.

The nice thing about `emit()` is that it also comes with parameter names, as provided in the `#[signal]` attribute. This lets IDEs provide
more context, e.g. show parameter inlay hints in `emit()` calls.

In addition to the specific `emit()` method, the `TypedSignal` (deref target of the custom signal type) also provides a generic method
`emit_tuple()`, which takes a tuple of all arguments, by value. This is rarely needed, but can be useful in situations where you want to pass
multiple arguments as a "bundle". Just for completeness, the above call is equivalent to:

```rust
self.signals().damage_taken().emit_tuple((amount,));
```


## Accessing signals outside the class

As your game grows in interactions, you may want to configure or emit signals not just within `impl Monster` blocks, but also from other parts
of your codebase. The trait method [`WithSignals::signals()`][api-withsignals] allows direct access from `&mut self`, but outside you often
only have a `Gd<Monster>`. You could technically `bind_mut()` that object, but there's a better way without borrow-checking.

For this reason, `Gd` itself [_also_ provides a `signals()` method][api-gd-signals], returning the exact same _signal collection_ API:

```rust
let monster: Gd<Monster> = ...;
let sig = monster.signals().damage_taken();
```


### Signal visibility

Like all items in Rust, signals are private by default, i.e. only visible in their module and submodules.
You can make them public by adding `pub` to the `#[signal]` attribute:

```rust
#[godot_api]
impl Monster {
    #[signal]
    pub fn damage_taken(amount: i32);
}
```

Of course, `pub(crate)`, `pub(super)` or `pub(in path)` are also possible for more fine-grained control.

```admonish warning title="Exceeding visibility"
`#[signal]` visibility **must not exceed** class visibility.

If you get errors such as "can't leak private type", then you violated this rule.
```

So, if your class is declared as `struct Monster` (private), then you cannot declare signals as `pub` or `pub(crate)`. This is due to a technical
limitation resulting from signals being separate types, which refer to the class type in their APIs. Making them "more public" than the class
would thus circumvent Rust's privacy rules.

Semantically, it makes sense though: the only situation where you'd need outside access is through `Gd<SomeClass>::signals()`, and this implies
that `SomeClass` is visible at that point. But unlike other Rust items such as `fn`, wider visibility isn't automatically limited to "at most
struct visibility", but causes a compile error.

Note that you cannot separate the visibility of connect and emit APIs. If you want to make sure that outsiders can only emit, keep the signal
private and provide a public wrapper function in your class that forwards the call to the signal.


### Connecting from outside

Let's say you have a sound system which should play a sound effect whenever a monster takes damage. You can connect to the signal from there:

```rust
impl SoundSystem {
    fn connect_sound_system(&self, monster: &Gd<Monster>) {
        let this = self.to_gd(); // Gd<SoundSystem>
        
        monster.signals()
            .damage_taken()
            .connect_obj(this, |s: &mut Self, _amount| {
                s.play_sound(Sfx::MonsterAttacked);
            });
    }
}
```


### Emitting from outside

Like connecting, emitting can also happen through `Gd::signals()`. The rest remains the same.

```rust
fn load_map() {
    // All the loading.
    ...

    // Notify player that the world around is now loaded.
    let player: Gd<Player> = ...;
    player.signals().on_world_loaded().emit();
}
```


## Advanced signal setups

The `TypedSignal::connect*()` methods are designed to be straightforward, while covering common use cases. If you need more advanced setups,
a high degree of customization is provided by [`TypedSignal::connect_builder()`][api-typedsignal-connectbuilder].

The returned `ConnectBuilder` provides several dimensions of configurability:

- Receiver: `function(args)`, `method(&self, args)`, `method(&mut self, args)`
- Provided object: none, `&mut self` or `Gd<T>`
- Connection flags: `DEFERRED`, `ONESHOT`, `PERSIST`
- Single-threaded (default) or thread-crossing (_sync_)

To finish it, `done()` is invoked. Some example setups:

```rust
// Connect -> Self::log_event(&self, event: String)
signal.connect_builder()
    .object_self() // pass in &self (the object surrounding the signal)
    .method_immut(Self::log_event) // receive &self
    .flags(ConnectFlags::DEFERRED | ConnectFlags::ONESHOT)
    .done();

// Connect -> Logger::log_event_mut(&mut self, event: String)
signal.connect_builder()
    .object(some_gd) // pass in Gd<T> (arbitrary object)
    .method_mut(Logger::log_event_mut) // receive &mut self
    .done();

// Connect -> Logger::log_event(event: String)
signal.connect_builder()
    .function(Logger::log_event) // associated fn, no receiver
    .sync() // allows another thread to receive signal (without panic)
    .done();
```

The builder methods need to be called in the correct order ("stages"). See [API docs][api-typedsignal-connectbuilder] for more information.


### Untyped signals

Godot's low-level APIs for dealing with untyped signals are still available:

- [`Object::connect()`][api-object-connect], `Object::connect_ex()`
- [`Object::emit_signal()`][api-object-emitsignal]
- [`Signal::connect()`][api-signal-connect]
- [`Signal::emit()`][api-signal-emit]

They can be used as a fallback for areas that the new typed signal API doesn't cover yet (e.g. Godot's built-in signals), or in situations
where you only have some information available at runtime.

In order to emit an untyped signal, you need to access a mutable reference to the Base property of your struct, as it inherits Object, letting you call its `emit_signal` method.
Considering the `Monster` struct from the previous examples, we could emit its signal with:
```rust
self.base_mut().emit_signal(
    "damage_taken",
    &[
        Variant::from(amount_damage_taken),
    ]
);
```

Certain typed-signal features are still planned and will make working with signals even more streamlined. Other features are likely not going
to be ported to godot-rust, e.g. a `Callable::bind()` equivalent for typed Rust methods. Just use closures instead.


## Conclusion

In this chapter, we saw how godot-rust's **type-safe signals** provide an intuitive and resilient way to deal with Godot's observer pattern
and avoid certain pitfalls of GDScript.
Rust function references or closures can be directly connected to signals, and emitting is achieved with regular function calls.


[api-object]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html
[api-signal]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#signals
[api-withsignals]: https://godot-rust.github.io/docs/gdext/master/godot/obj/trait.WithSignals.html
[api-gd-signals]: https://godot-rust.github.io/docs/gdext/master/godot/obj/struct.Gd.html#method.signals
[godot-gdscript-signals]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#signals
[api-typedsignal]: https://godot-rust.github.io/docs/gdext/master/godot/register/struct.TypedSignal.html
[api-typedsignal-connectbuilder]: https://godot-rust.github.io/docs/gdext/master/godot/register/struct.TypedSignal.html#method.connect_builder

[api-object-connect]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html#method.connect
[api-object-emitsignal]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html#method.emit_signal
[api-signal-connect]: https://godot-rust.github.io/docs/gdext/master/godot/builtin/struct.Signal.html#method.connect
[api-signal-emit]: https://godot-rust.github.io/docs/gdext/master/godot/builtin/struct.Signal.html#method.emit
