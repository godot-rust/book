# Futures and Deferred Functions

Execute logic using a future or a deferred function at the end of the frame.

## Running Deferred logic

Sometimes it is useful to run logic after all of the other logic of the other
nodes has complete.  While you can ask the
[godot engine to execute exported Rust functions](https://godot-rust.github.io/gdnative-book/bind/calling-gdscript.html#function-calls)
, godot-rust also provides a type-safe way to defer executed logic to the next
frame.

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
   base: Base<Node>,
}

#[godot_api]
impl Game {
  fn now_and_later(&mut self) {
      godot_print!("This was run at the beginning of the frame!");
      self.run_deferred(|_this| {
          godot_print!("This was run at the end of the frame!")
      });
  }
}
```

## Futures

Rust's futures are fully supported for integrating asynchronous programming.
The key point is that you will need you will need a Godot pointer that can be passed
to `godot::task::spawn`.

```admonish note title="Connecting signals"
The only way to connect a signal in Rust so that the callback method
is called with a Godot pointer is to use the signal builder.
```

```rust
use godot::prelude::*;
use godot::classes::{ Area2D };

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Game {
   base: Base<Node>,
}

#[godot_api]
impl INode for Game {
    fn ready(&mut self) {
        // Connect signal: $Player.body_entered -> Self::show_messages.
        self.base()
            .get_node_as::<Area2D>("Player")
            .signals()
            .body_entered()
            .builder()
            .connect_other_gd(self, Self::show_messages);
    }
}

#[godot_api]
impl Game {
    // Async function that implements sleep using Godot timers.
    async fn sleep(&self, duration: f64) {
        let timer = self.base().get_tree().create_timer(duration);
        // Use a future to wait for the timeout signal.
        timer.signals().timeout().to_future().await;
    }

    // Show one message immediately, and other after one second.
    #[func(gd_self)] // Also allow attaching the callback with the Godot editor.
    fn show_messages(this: Gd<Self>, _area: Gd<Node2D>) {
        godot::task::spawn(async move {
            godot_print!("Immediate message!");
            this.bind().sleep(1.0).await;
            godot_print!("Message after one second!")
        });
    }
}
```

```admonish warning title="Getting a Godot pointer in a class method"
While [it is possible to get a Godot pointer inside of a class method](https://godot-rust.github.io/book/register/functions.html?highlight=bind_mut#calling-rust-methods-binds), `bind()` and
`bind_mut()` will not be able to return a guarded object as it is ready
implicity bound for the method call.
```

```rust
#[godot_api]
impl Game {
    fn crash(&mut self) {
        // FAIL! While you can have multiple smart pointers, you can't bind
        // multiple items. This will not release an existing guard.
        let gd = self.object_to_owned();

        // FAIL! You cannot use drop to free a guard on a borrowed object.
        std::mem::drop(self); // Doesn't do anything

        godot::task::spawn(async move {
            // FAIL! `drop` did nothing so this will crash your program!
            gd.bind_mut();
        });
    }

}
```

```admonish warning title="Joining threads"
Because the Godot engine is running separate of Rust, you will want
to be thoughtful on using any native Rust calls for joining async calls
to avoid a deadlock freezing your program.
```

```rust
#[godot_api]
impl Game {
    async fn sleep(&self, duration: f64) {
        let timer = self.base().get_tree().create_timer(duration);
        timer.signals().timeout().to_future().await;
    }

    fn freeze_the_program(&mut self) {
        // FAIL! This will cause a deadlock that will freeze your program
        // where Rust and Godot are waiting for each other!
        futures::executor::block_on(self.sleep(1.0));
    }

}
```
