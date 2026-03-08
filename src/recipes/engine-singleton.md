<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# User singletons

It is important for you to understand the [Singleton pattern][singleton] to
properly utilize this system.

```admonish info title="Controversy"
The "Singleton pattern" is often referred to as an anti-pattern, because it violates several good practices for clean, modular code. However, it is
also a tool that can be used to solve certain design problems. As such, it is used internally by Godot, and is available to godot-rust
users as well.

Read more about criticisms [here][singleton-crit].
```

Custom singletons in Godot:

- are `Object` types
- always run in the editor (implied `#[class(tool)]`)
- are always accessible to GDScript and GDExtension languages
- must be registered and unregistered in the `InitStage::Scene` step

Godot provides _many_ built-in singletons in its API. You can find a full list [here][godot-singleton-list].

[godot-singleton-list]: https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#properties
[singleton-crit]: https://en.wikipedia.org/wiki/Singleton_pattern#Criticism
[singleton]: https://en.wikipedia.org/wiki/Singleton_pattern


## Table of contents

<!-- toc -->


## Registering custom singleton

You can register given class as a Singleton with `#[class(singleton)]`.

```rust
#[derive(GodotClass)]
#[class(init, singleton)]
struct MySingleton {
    // For `#[class(singleton)]`, the default base is Object, not RefCounted.
    base: Base<Object>,
}

// Can be accessed like any other singleton from the main thread.
let val = MySingleton::singleton().bind().foo();
```

Now that your singleton is available (and once you've recompiled and reloaded), you should be able to access it from GDScript like so:

```php
extends Node

func _ready() -> void:
    MySingleton.foo()
```


## Using a singleton with `on_main_loop_frame`

Since Godot4.5+ [`on_main_loop_frame`][api-main-loop-frame] can be used to invoke an user singleton:

```rs
fn global_delta() -> f64 {
    let ticks = ProjectSettings::singleton()
        .get("physics/common/physics_ticks_per_second")
        .to::<i64>();
    1.0 / (ticks as f64)
}

#[derive(GodotClass)]
#[class(init, singleton)]
struct MySingleton {
    #[init(val = global_delta())]
    delta: f64,
    
    #[init(val = true)]
    paused: bool,
    
    #[init(val = Instant::now())]
    time: Instant,
    
    base: Base<Object>,
}

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_main_loop_frame() {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        MySingleton::singleton().bind_mut().frame();
    }
}

impl MySingleton {
    pub fn frame(&mut self) {
        if self.paused {
            return;
        }

        let elapsed = self.time.elapsed().as_secs_f64();
        self.elapsed += elapsed;

        if self.elapsed >= self.delta {
            let time_scale = Engine::singleton().get_time_scale();
            self.run_simulation(self.elapsed * time_scale);
            self.elapsed = 0.0;
        }
        self.time = Instant::now();
    }
}
```

[api-main-loop-frame]: https://godot-rust.github.io/docs/gdext/master/godot/init/trait.ExtensionLibrary.html#method.on_main_loop_frame


## Registering custom singleton without proc macro

Custom singleton can be registered through [`godot::classes::Engine`][api-class-engine].
Additionally, implementing [`UserSingleton`][api-user-singleton] allows accessing a registered singleton instance through `singleton()`.

User singletons should be registered under their class name – otherwise some Godot components (for example GDScript before 4.4)
might have trouble handling them, and the editor might crash when using `T::singleton()`.

There should be only one instance of a given singleton class in the engine, valid as long as the library is loaded.
Therefore, user singletons are limited to classes with manual memory management (ones not inheriting from RefCounted).

```rs
#[derive(GodotClass)]
#[class(init, base = Object)]
struct MySingleton {}

// Provides blanket implementation allowing to use MySingleton::singleton().
// Ensures that `MySingleton` is a valid singleton 
// (i.e., a non-refcounted GodotClass).
impl UserSingleton for MySingleton {}

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_stage_init(stage: InitStage) {
        // Singleton should be registered before the MainLoop startup;
        // otherwise it won't be recognized by the GDScriptParser.
        if stage == InitStage::Scene {
            let obj = MySingleton::new_alloc();
            Engine::singleton()
                .register_singleton(
                    &MySingleton::class_id().to_string_name(), 
                    &obj
                );
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            let obj = MySingleton::singleton();
            Engine::singleton()
                .unregister_singleton(
                    &MySingleton::class_id().to_string_name()
                );
            obj.free();
        }
    }
}
```

[api-user-singleton]: https:/godot-rust.github.io/docs/gdext/master/godot/obj/trait.UserSingleton.html
[api-class-engine]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Engine.html


## Singletons and the `SceneTree`

Singletons cannot safely access the scene tree. At any given moment, they may exist without a scene tree being active.
While it is technically possible to access the tree through hacky methods, it is **highly recommended** to use a
custom `EditorPlugin` for this purpose. Creating an `EditorPlugin` allows for registering an "autoload singleton" which is a `Node` (or
 derived) type and is automatically loaded into the `SceneTree` by Godot when the game starts.
