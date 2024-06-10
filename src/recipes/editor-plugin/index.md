<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Editor plugins


Using `EditorPlugin` types is very similar to the process used when [writing plugins in GDScript][gd-plugins].
Unlike GDScript plugins, godot-rust plugins are registered automatically and cannot be enabled/disabled in the
Project Settings plugins pane.

Plugins written in GDScript are automatically disabled if they have a code error, but because Rust is a compiled language,
you cannot introduce compile-time errors.

[gd-plugins]: https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html


## Creating an `EditorPlugin`

```rust
#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct MyEditorPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for MyEditorPlugin {
    fn enter_tree(&mut self) {
        // Perform typical plugin operations here.
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin operations here.
    }
}
```

Since this is an `EditorPlugin`, it will be automatically added to the scene tree root. This means it can access the scene tree
at runtime. Additionally, it is safe to access the `EditorInterface` singleton through this node,
which allows adding different GUI elements to the editor directly. This can be helpful if you have an
advanced GUI you want to implement.

<!-- TODO: more plugins from https://docs.godotengine.org/en/stable/tutorials/plugins/editor/index.html -->