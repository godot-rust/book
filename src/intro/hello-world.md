<!--
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->


# Hello World

This page shows you how to develop your own small extension library and load it from Godot.
The tutorial is heavily inspired by [Creating your first script][tutorial-begin] from the official Godot documentation.
It is recommended to follow that alongside this tutorial, in case you're interested how certain GDScript concepts map to Rust.


## Table of contents
<!-- toc -->


## Connecting Godot and Rust

First things first.  
For the following objective, we'll first make

- A Cargo crate of your own;
- A Godot project created;

You can very much start from an already existing Godot project. Here we'll start from scratch for the sake of completeness.


### Creating the Godot Project

For this example we're going to create each of the items separated into neighboring folders in the same parent folder. You'll be able to choose
locations of your own as we progress. Let's start by creating a blank Godot project if you haven't already.

Not much to do here yet, we'll come back to it later.


### Creating the Rust Cargo crate

To create a Cargo crate, have your terminal open, navigate to your desired folder and then call `cargo` to create it by typing:

```bash
cargo new "your_crate_name" --lib
```

The argument `--lib` will tell `cargo` to create the package from a Library template. From there, we're going to configure it in the file which stores
its metadata, `Cargo.toml`. This file stores the information we know of the library package and what it uses in order to work with it.

Open `Cargo.toml` in your favorite text editor and edit it as the following:

```toml
[package]
name = "rust_project" # Will appear in the filename of the dynamic Rust library you compile.
version = "0.1.0" # version of your library (your criteria)
edition = "2021" # Rust edition version (remains unchanged)

[lib]
crate-type = ["cdylib"] # defines the kind of library it will become

[dependencies] # lists what your own library needs in order to function
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
```


#### Why "`cdylib`"?
<!-- editors note: sole reason this header's here is because this marks a quick spot to go back to later on,
specially if a new reader is just scrolling by around the book. -->

The `cdylib` crate type is a bit unusual in Rust. Instead of building a binary (`bin`, application) or a library to be utilized by other Rust code
(`lib`), we create a _dynamic_ library, exposing an interface in the C programming language. This dynamic library is loaded by Godot at runtime,
through the GDExtension interface.

```admonish note
The main crate of gdext is called `godot`. At this point, it is still hosted on GitHub; in the future, it will be published to crates.io. To fetch the
latest changes, you can regularly run a `cargo update` (possibly breaking). Keep your `Cargo.lock` file under version control, so that it's easy to
revert updates.
```

To compile each iteration of the extension as you write your code, you can use `cargo` as you normally do with any other Rust project:

```bash
cargo build
```

This should output to `rust_project/target/debug/` at least one variation of a compiled library depending on your setup. For instance,
`librust_project.so` is created if you're on Linux:

```log
$ cargo build
   Compiling godot4-prebuilt v0.0.0 (https://github.com/godot-rust/godot4-prebuilt?branch=4.1.1#fca6897d)
   Compiling proc-macro2 v1.0.69
   [...]
   Compiling godot v0.1.0 (https://github.com/godot-rust/gdext?branch=master#66df8f47)
   Compiling rust_project v0.1.0 (/home/wilker/Documentos/Projects/Programming/Godot/Tutorial/rust_project)
    Finished dev [unoptimized + debuginfo] target(s) in 1m 46s

$ ls -l
╭───┬─────────────────────────────────┬──────┬─────────┬────────────────╮
│ # │              name               │ type │  size   │    modified    │
├───┼─────────────────────────────────┼──────┼─────────┼────────────────┤
│ 0 │ target/debug/build              │ dir  │   648 B │ 2 minutes ago  │
│ 1 │ target/debug/deps               │ dir  │  3.8 KB │ now            │
│ 2 │ target/debug/examples           │ dir  │     0 B │ 2 minutes ago  │
│ 3 │ target/debug/incremental        │ dir  │    52 B │ 16 seconds ago │
│ 4 │ target/debug/librust_project.d  │ file │   190 B │ now            │
│ ⮞ │ target/debug/librust_project.so │ file │ 62.6 MB │ now            │
╰───┴─────────────────────────────────┴──────┴─────────┴────────────────╯
```


### `.gdextension` file

The job of this file is to provide Godot with pointers to a library that is compiled elsewhere. We're going to use it to connect Godot to your
compiled library so it can be used within the context of your Godot project.

First, add an empty `.gdextension` file somewhere in your project folder, which is the equivalent of `.gdnlib` for GDNative. In this case we're
creating `res://HelloWorld.gdextension` in the root folder of the project. Following along,

```ini
[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
linux.debug.x86_64 = "res://../{myCrate}/target/debug/lib{myCrate}.so"
```

- The `[configuration]` section should be copied as-is.
  - The name `gdext_rust_init` for the entry point is provided by **godot-rust**
    and is required for the extension to initialize;
  - `compatibility_minimum` is the minimum version of **Godot** required by your extension to work.
    Opening the project with a version of Godot lower than this will prevent your extension from running;
- The `[libraries]` section should be updated to match the paths of your dynamic Rust libraries.
  - `linux.debug.x86_64` is the [**target build**](#other-possible-target-paths) of the **Godot** project.
  - `{myCrate}` should be replaced with the name of your compiled file. For instance,
    `librust_project.so` from the name of the library we compiled earlier;

```admonish note
For exporting your project, you'll need to use paths inside `res://`. This prefix represents the path to your files **relative to the root folder of
your project**. You can learn more about Godot's resource paths [here][ref-resource-paths].
```


#### Other possible target paths

The following is a list of other possible paths depending on the target of your library. You can copy them as-is for this example, replacing
`{myCrate}` with the name of your actual library:

```ini
[configuration] # same as before
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1

[libraries]
linux.debug.x86_64 =     "res://../{myCrate}/target/debug/lib{myCrate}.so"
linux.release.x86_64 =   "res://../{myCrate}/target/release/lib{myCrate}.so"
windows.debug.x86_64 =   "res://../{myCrate}/target/debug/{myCrate}.dll"
windows.release.x86_64 = "res://../{myCrate}/target/release/{myCrate}.dll"
macos.debug =            "res://../{myCrate}/target/debug/lib{myCrate}.dylib"
macos.release =          "res://../{myCrate}/target/release/lib{myCrate}.dylib"
macos.debug.arm64 =      "res://../{myCrate}/target/debug/lib{myCrate}.dylib"
macos.release.arm64 =    "res://../{myCrate}/target/release/lib{myCrate}.dylib"
```

The `..` is a pointer to a parent folder of the end path.
e.g Godot will look in a path relative to this project directory's parent folder.


```admonish tip
You can also employ the use of symbolic links and git submodules and then treat those as regular folders and files. Godot reads those just fine too! 
```

```admonish note
If you specify your cargo compilation target via the `--target` flag or a `.cargo/config.toml` file, the rust library will be placed in a 
path name that includes target architecture, and the `.gdextension` library paths will need to match. For example, for M1 Macs 
(`macos.debug.arm64` and `macos.release.arm64`), the path would be `"res://../rust/target/aarch64-apple-darwin/debug/lib{myCrate}.dylib"`.
```


### `extension_list.cfg`

A second file `res://.godot/extension_list.cfg` should be generated once you open the Godot editor for the first time.
If not, you can also manually create it, simply containing the Godot path to your `.gdextension` file:

```text
res://HelloWorld.gdextension
```
<!-- TODO: explain further the necessity and functionality of the extension_list.cfg -->


## Implementing your extension and your first Node


### Rust entry point

[Like mentioned earlier](#why-cdylib), our compiled C library needs to expose an _entry point_ to Godot: a C function that can be called through
the GDExtension. Setting this up requires quite some low-level [FFI][wikipedia-ffi] code, which is why gdext abstracts it for you.

In your `lib.rs`, replace the template with the following:

```rust
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
```

There are multiple things going on here:

1. Place the [`prelude`][api-prelude] module from the [`godot`][api-godot] crate into scope.
   This module contains the most common symbols in the gdext API.
2. Define a struct called `MyExtension`. This is just a type tag without data or methods, you can name it however you like.
3. Implement the [`ExtensionLibrary`][api-extensionlibrary] trait for our type, and mark it with the `#[gdextension]` attribute.

The last point declares the actual GDExtension entry point, and the proc-macro attribute takes care of the low-level details.


#### "Does it load though?"

If you want to see Godot loading your extension, you can override `on_level_init(InitLevel)` from the
[`ExtensionLibrary`][api-extensionlibrary] trait like so:

```rust
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        godot_print!("Hello Library!");

        godot_print!("Initialization level: {:?}", level);
    }
}
```

After `cargo build`, Godot should display the following messages when running it from the command line:


```log
Godot/Tutorial/GodotProject
$ godot4 -e
Initialize GDExtension API for Rust: Godot Engine v4.2.beta1.official
Hello Library!
Initialization level: Core
Godot Engine v4.2.beta1.official.b1371806a - https://godotengine.org
Hello Library!
Initialization level: Servers
Vulkan API 1.3.246 - Forward+ - Using Vulkan Device #0: AMD - AMD Radeon Vega 8 Graphics (RADV RAVEN)

Hello Library!
Initialization level: Scene
Hello Library!
Initialization level: Editor
```

```admonish note
These messages will most likely not show up unless you're either launching [Godot editor from your command line][godot-command-line] by using
`godot -e` like in this example, or making your gdextension file visible for the first time in Godot while opened.
```


### Your first Rust class

Now, let's write Rust code to define a _class_ that can be used in Godot.

Every class inherits an existing Godot-provided class (its _base class_ or just _base_). Rust does not natively support inheritance, but the gdext API
emulates it to a certain extent.


#### Class declaration

In this example, we declare a class called `Player`, which inherits [`Sprite2D`][api-sprite2d] (a node type):

```rust
use godot::prelude::*;
use godot::engine::Sprite2D;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>
}
```

Let's break this down.

1. The gdext prelude contains the most common symbols. Less frequent classes are located in the [`engine`][api-engine] module.

2. The `#[derive]` attribute registers `Player` as a class in the Godot engine.
   See [API docs][api-derive-godotclass] for details about `#[derive(GodotClass)]`.

   ```admonish info
   `#[derive(GodotClass)]` _automatically_ registers the class -- you don't need an explicit 
   `add_class()` registration call, or a `.gdns` file as it was the case with GDNative.
   
   You will, however, need to restart the Godot editor for it to take effect.
   ```

3. We define two fields `speed` and `angular_speed` for the logic. These are regular Rust fields, no magic involved. More about their use later.

4. The optional `#[class]` attribute configures how the class is registered. In this case, we specify that `Player` inherits Godot's
   `Sprite2D` class. If you don't specify the `base` key, the base class will implicitly be `RefCounted`, just as if you omitted the
   `extends` keyword in GDScript.


5. The `#[base]` attribute declares the `sprite` field, which allows `self` to access the base instance (via composition, as Rust does not have
   native inheritance).

   - The field must have type `Base<T>`.
     - `T` must match the declared base class, e.g. `Sprite2D` for `#[class(base=Sprite2D)]` becomes `Base<Sprite2D>`.
   - The name can be freely chosen. Here it's `sprite`, but `base` is also a common convention.
   - You do not _have to_ declare this field. If it is absent, you cannot access the base object from within `self`.
     This is often not a problem, e.g. in data bundles inheriting `RefCounted`.

```admonish warning
When adding an instance of your `Player` class to the scene, make sure to select node type `Player` and not its base `Sprite2D`.
Otherwise, your Rust logic will not run.

If Godot fails to load a Rust class (e.g. due to an error in your extension), it may silently replace it with its base class.
Use version control (git) to check for unwanted changes in `.tscn` files.
```


#### Method declaration

Now let's add some logic. We start with overriding the `init` method, also known as the constructor.
This corresponds to GDScript's `_init()` function.

```rust
use godot::engine::Sprite2DVirtual;

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console
        
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite
        }
    }
}
```

Again, those are multiple pieces working together, let's go through them one by one.

1. `#[godot_api]` - this lets gdext know that the following `impl` block is part of the Rust API to expose to Godot.
   This attribute is required here; accidentally forgetting it will cause a compile error.

2. `impl Sprite2DVirtual` - each of the engine classes has a `{ClassName}Virtual` trait, which comes with virtual functions for that
   specific class, as well as general-purpose functionality such as `init` (the constructor) or `to_string` (String conversion).
   The trait has no required methods.

3. The `init` constructor is an associated function ("static method" in other languages) that takes the base instance as argument and returns
   a constructed instance of `Self`. While the base is usually just forwarded, the constructor is the place to initialize all your other fields.
   In this example, we assign initial values `400.0` and `PI`.

Now that initialization is sorted out, we can move on to actual logic. We would like to continuously rotate the sprite, and thus override
the `process()` method. This corresponds to GDScript's `_process()`. If you need a fixed framerate, use `physics_process()` instead.

```rust
use godot::engine::Sprite2DVirtual;

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(base: Base<Sprite2D>) -> Self { /* as before */ }

    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        
        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}
```

GDScript uses property syntax here; Rust requires explicit method calls instead. Also, access to base class methods -- such as `rotate()` in
this example -- is done via the `#[base]` field.

This is a point where you can compile your code, launch Godot and see the result. The sprite should rotate at a constant speed.

![rotating sprite][img-sprite-rotating]

```admonish tip
**Launching the Godot application**

While it's possible to open the Godot editor and press the launch button every time you made a change in Rust, this is not the most efficient
workflow. Unfortunately there is [a GDExtension limitation][issue-no-reload] that prevents recompilation while the editor is open 
(at least on Windows systems -- it tends to work better on Linux and macOS).

However, if you don't need to modify anything in the editor itself, you can launch Godot from the command line or even your IDE.
Check out the [command-line tutorial][godot-command-line] for more information.
```

We now add a translation component to the sprite, following [the upstream tutorial][tutorial-full-script].

```rust
use godot::engine::Sprite2DVirtual;

#[godot_api]
impl Sprite2DVirtual for Player {
    fn init(base: Base<Sprite2D>) -> Self { /* as before */ }

    fn physics_process(&mut self, delta: f64) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta
        
        self.sprite.rotate((self.angular_speed * delta) as f32);

        let rotation = self.sprite.get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.sprite.translate(velocity * delta as f32);
        
        // or verbose: 
        // self.sprite.set_position(
        //     self.sprite.get_position() + velocity * delta as f32
        // );
    }
}
```

The result should be a sprite that rotates with an offset.

![rotating translated sprite][img-sprite-moving]


### Custom Rust APIs

Say you want to add some functionality to your `Player` class, which can be called from GDScript. For this, you have a separate `impl` block,
again annotated with `#[godot_api]`. However, this time we are using an _inherent_ `impl` (i.e. without a trait name).

Concretely, we add a function to increase the speed, and a signal to notify other objects of the speed change.

```rust
#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.sprite.emit_signal("speed_increased".into(), &[]);
    }

    #[signal]
    fn speed_increased();
}
```

`#[godot_api]` takes again the role of exposing the API to the Godot engine. But there are also two new attributes:

- `#[func]` exposes a function to Godot. The parameters and return types are mapped to their corresponding GDScript types.
- `#[signal]` declares a signal. A signal can be emitted with the `emit_signal` method (which every Godot class provides, since it is inherited
  from `Object`).

API attributes typically follow the GDScript keyword names: `class`, `func`, `signal`, `export`, `var`, ...

That's it for the _Hello World_ tutorial! The following chapters will go into more detail about the various features that gdext provides.


[api-derive-godotclass]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/derive.GodotClass.html
[api-engine]: https://godot-rust.github.io/docs/gdext/master/godot/engine/index.html
[api-extensionlibrary]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/trait.ExtensionLibrary.html
[api-godot]: https://godot-rust.github.io/docs/gdext/master/godot/index.html
[api-sprite2d]: https://godot-rust.github.io/docs/gdext/master/godot/engine/struct.Sprite2D.html
[api-prelude]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/index.html
[ref-resource-paths]: https://docs.godotengine.org/en/stable/tutorials/scripting/resources.html#external-vs-built-in
[tutorial-begin]: https://docs.godotengine.org/en/stable/getting_started/step_by_step/scripting_first_script.html
[tutorial-full-script]: https://docs.godotengine.org/en/stable/getting_started/step_by_step/scripting_first_script.html#complete-script
[img-sprite-rotating]: https://docs.godotengine.org/en/stable/_images/scripting_first_script_godot_turning_in_place.gif
[img-sprite-moving]: https://docs.godotengine.org/en/stable/_images/scripting_first_script_rotating_godot.gif
[issue-no-reload]: https://github.com/godotengine/godot/issues/66231
"A screenshot of the GUI of the Godot project manager about to create a new project"
[wikipedia-ffi]: https://en.wikipedia.org/wiki/Foreign_function_interface
"Foreign Function Interface"
