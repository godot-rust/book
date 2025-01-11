<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Recipes


## Custom resources

With godot-rust, you are able to define custom `Resource` classes which are then available to the end user.


## Editor plugins

`EditorPlugin` types are loaded during editor and runtime and are able to access the editor as well as the scene tree. This type follows the same
functionality that a typical `EditorPlugin` class written in GDScript would, but crucially with access to the _entire Rust ecosystem_.


## Engine singletons

An Engine Singleton is a class instance that is always globally available (following the Singleton pattern). However,
it cannot access the `SceneTree` through any reliable means.


## `ResourceFormatSaver` and `ResourceFormatLoader`

Provide custom logic for saving and loading your `Resource` derived classes.


## Custom icons

Adding custom icons to your classes is actually fairly simple!
