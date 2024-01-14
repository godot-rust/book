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

```rs
#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct MyEditorPlugin {
    #[base]
    base: Base<EditorPlugin>,
}

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

```admonish hint title="Gameplay-only code"
Use an [`is_editor_hint` guard][api-engine-iseditorhint] if you don't want some code executing during runtime of the game.

[Read more information on guard clauses in computer science.][wiki-guard-csci]

[api-engine-iseditorhint]: https://godot-rust.github.io/docs/gdext/master/godot/engine/struct.Engine.html#method.is_editor_hint
[wiki-guard-csci]: https://en.wikipedia.org/wiki/Guard_(computer_science)
