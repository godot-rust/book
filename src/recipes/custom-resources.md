<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Custom resources

Custom `Resource`s are exposed to the end user to use within their development. `Resource`s can store data that is easily edited from within
the editor GUI. For example, you can create a custom `AudioStream` type that handles a new and interesting audio file type.


## Registering a `Resource`

This workflow is similar to the [Hello World example][hello]:

```rust
#[derive(GodotClass)]
#[class(init, base=Resource)]
struct ResourceType {
    base: Base<Resource>,
}
```

The above resource does not export any variables. While not all resources require exported variables, most do.

If your custom resource has lifecycle methods (`ready()`, `process()` etc.) that need to run in the editor,
you should annotate the class with `#[class(tool)]`.

```rust
#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
struct ResourceType {
    base: Base<Resource>,
}

#[godot_api]
impl IResource for ResourceType {
    fn ready(&mut self) { ... }
    fn process(&mut self, delta: f64) { ... }
}

```

It is important that similar to defining custom resources in GDScript, marking this class as a "tool class"
is required to be usable within the editor.

The systems for registering functions, properties, and more are described in detail in the
[Registering Rust symbols][register] section.

[hello]: ../intro/hello-world.md
[register]: ../register/index.html
