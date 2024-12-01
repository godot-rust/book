<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Engine singletons

It is important for you to understand the [Singleton pattern][singleton] to
properly utilize this system.

```admonish info title="Controversy"
The "Singleton pattern" is often referred to as an anti-pattern, because it violates several good practices for clean, modular code. However, it is
also a tool that can be used to solve certain design problems. As such, it is used internally by Godot, and is available to godot-rust
users as well.

Read more about criticisms [here][singleton-crit].
```

An engine singleton is registered through [`godot::classes::Engine`][api-class-engine].

Custom engine singletons in Godot:

- are `Object` types
- are always accessible to GDScript and GDExtension languages
- must be manually registered and unregistered in the `InitLevel::Scene` step

Godot provides _many_ built-in singletons in its API. You can find a full list [here][godot-singleton-list].

[api-class-engine]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Engine.html
[godot-singleton-list]: https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#properties
[singleton-crit]: https://en.wikipedia.org/wiki/Singleton_pattern#Criticism
[singleton]: https://en.wikipedia.org/wiki/Singleton_pattern


## Table of contents

<!-- toc -->


## Defining a singleton

Defining a singleton is the same as registering a custom class.

```rust
#[derive(GodotClass)]
#[class(init, base=Object)]
struct MyEditorSingleton {
    base: Base<Object>,
}

#[godot_api]
impl MyEditorSingleton {
    #[func]
    fn foo(&mut self) {}
}
```


## Registering a singleton

Registering singletons is done during the `InitLevel::Scene` stage of initialization.

To achieve this, we can customize our init/shutdown routines by overriding `ExtensionLibrary` trait methods.

```rust
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The `&str` identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton(
                "MyEngineSingleton",
                &MyEngineSingleton::new_alloc(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Let's keep a variable of our Engine singleton instance,
            // and MyEngineSingleton name.
            let mut engine = Engine::singleton();
            let singleton_name = "MyEngineSingleton";

            // Here, we manually retrieve our singleton(s) that we've registered,
            // so we can unregister them and free them from memory - unregistering
            // singletons isn't handled automatically by the library.
            if let Some(my_singleton) = engine.get_singleton(singleton_name) {
                // Unregistering from Godot, and freeing from memory is required
                // to avoid memory leaks, warnings, and hot reloading problems.
                engine.unregister_singleton(singleton_name);
                my_singleton.free();
            } else {
                // You can either recover, or panic from here.
                godot_error!("Failed to get singleton");
            }
        }
    }
}
```

```admonish warning title="Singletons inheriting from *RefCounted*"
Use a manually-managed class as a base (often `Object` will be enough) for custom singletons to avoid prematurely freeing the object.
If for any reason you need to have an instance of a reference-counted object registered as a singleton, this
[issue thread][refcounted-singleton-issue] presents some possible workarounds.
```

[refcounted-singleton-issue]: https://github.com/godot-rust/gdext/issues/522


## Calling from GDScript

Now that your singleton is available (and once you've recompiled and reloaded), you should be able to access it from GDScript like so:

```php
extends Node

func _ready() -> void:
    MyEditorSingleton.foo()
```


## Calling from Rust

You may also want to access your singleton from Rust as well.

```rust
godot::classes::Engine::singleton()
    .get_singleton(StringName::from("MyEditorSingleton"));
```

For more information on this method, refer to [the API docs][method-get-singleton].

[method-get-singleton]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Engine.html#method.get_singleton


## Singletons and the `SceneTree`

Singletons cannot safely access the scene tree. At any given moment, they may exist without a scene tree being active.
While it is technically possible to access the tree through hacky methods, it is **highly recommended** to use a
custom `EditorPlugin` for this purpose. Creating an `EditorPlugin` allows for registering an "autoload singleton" which is a `Node` (or
 derived) type and is automatically loaded into the `SceneTree` by Godot when the game starts.
