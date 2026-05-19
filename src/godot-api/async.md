<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Async Programming

Execute logic using [futures](https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html)
or deferred functions.


## Running Deferred Logic

Sometimes it is useful to run logic after all other node processing has completed.
While the Godot engine can
[execute exported Rust functions with `call_deferred`](https://godot-rust.github.io/gdnative-book/bind/calling-gdscript.html#function-calls),
godot-rust also provides a type-safe way to defer execution until the next frame
with `run_deferred`.

```rust
use godot::prelud::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
    base: Base<Node>,
}

#[godot_api]
impl Game {
    fn now_and_later(&mut self) {
        godot_print!("This runs at the beginning of the frame!");

        self.run_deferred(|_this| {
            godot_print!("This runs at the end of the frame!");
        });
    }
}
````


## Futures

Rust futures are fully supported for asynchronous programming.
The key requirement is that a Godot pointer is passed to
`godot::task::spawn`. For this reason,
connected callbacks should accept a Godot pointer
instead of the base class directly.

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
    base: Base<Node>,
}

#[godot_api]
impl Game {
    async fn sleep(&self, duration: f64) {
        // Your logic to sleep here
    }

    /// Show one message immediately and another after one second.
    #[func(gd_self)]
    fn show_messages(this: Gd<Self>, _area: Gd<Node2D>) {
        godot::task::spawn(async move {
            godot_print!("Immediate message!");

            this.bind().sleep(1.0).await;

            godot_print!("Message after one second!");
        });
    }
}
```

While it is possible to obtain
[a Godot pointer](https://godot-rust.github.io/book/register/functions.html?highlight=bind_mut#calling-rust-methods-binds),
inside a class method `bind()` and `bind_mut()` cannot return a guarded object
if the instance is already implicitly bound for the current method call.

The following example demonstrates how this can fail.

```rust
#[godot_api]
impl Game {
    fn crash_the_program(&mut self) {
        let gd = self.to_gd();

        // `drop` is a no-op on a borrowed value.
        std::mem::drop(self);

        godot::task::spawn(async move {
            // `self` already has an implicit `bind_mut()` from the method call.
            // Attempting to bind again will crash the program.
            gd.bind_mut();
        });
    }
}
```

Additionally, because Rust and Godot run in the same process (and often on the
same thread), blocking the thread while waiting for a future will freeze the
program.

```rust
#[godot_api]
impl Game {
    async fn sleep(&self, duration: f64) {
        // Your logic to sleep here
    }

    fn freeze_the_program(&mut self) {
        // The program freezes when `block_on()` is called here.
        //
        // Instead, use `godot::task::spawn()` to start async tasks.
        futures::executor::block_on(self.sleep(1.0));
    }
}
```
