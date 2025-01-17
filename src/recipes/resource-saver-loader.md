<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# `Resource` savers and loaders

The [`ResourceFormatSaver`][godot-saver] and [`ResourceFormatLoader`][godot-loader] classes allow you to serialize and deserialize your Rust
`Resource`-derived classes with a custom procedure, as well as define new recognized file extensions. This is mostly useful if you have resources
that contain _pure Rust state_. "Pure" in this context refers to members of your struct that don’t have any `#[var]` or similar annotations, i.e.
Godot isn't aware of them. This can easily be the case when you work with Rust libraries.

The following example gives you a starting point to copy-and-paste. For advanced use cases, consult the Godot documentation for these classes.

First of all, you need to call the provided functions in your library entry point at the `InitLevel::Scene`. This ensures proper initialization
and cleanup of your loader/saver.

```rust
// These imports will be needed across the following code samples.
use godot::classes::{
    Engine, IResourceFormatLoader, IResourceFormatSaver, ResourceFormatLoader,
    ResourceFormatSaver, ResourceLoader, ResourceSaver,
};
use godot::prelude::*;

#[gdextension]
unsafe impl ExtensionLibrary for MyGDExtension {
    // Register the singleton when the extension is loading.
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().register_singleton(
                "MyAssetSingleton",
                &MyAssetSingleton::new_alloc(),
            );
        }
    }

    // Unregisters the singleton when the extension exits.
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton().unregister_singleton("MyAssetSingleton");
            my_singleton.free();
        }
    }
}
```

Define the singleton to keep track of your Loaders and Savers.

```rust
// The definition of the singleton with all your loader/savers as members,
// to keep the object references for destruction later.
#[derive(GodotClass)]
#[class(base=Object, tool)]
struct MyAssetSingleton {
    base: Base<Object>,
    loader: Gd<MyAssetLoader>,
    saver: Gd<MyAssetSaver>,
}

#[godot_api]
impl IObject for MyAssetSingleton {
    fn init(base: Base<Object>) -> Self {
        let saver = MyAssetSaver::new_gd();
        let loader = MyAssetLoader::new_gd();
        
        // Register the loader and saver in Godot.
        //
        // If you want your default extension to be the one defined by your loader,
        // set the `at_front` parameter to true. Otherwise you can also remove the 
        // builder. Godot currently doesn't provide a way to completely deactivate 
        // the built-in loaders. 
        // WARNING: The built-in loaders won't work if you have _pure Rust state_.
        ResourceSaver::singleton().add_resource_format_saver_ex(&saver)
            .at_front(false)
            .done();
        ResourceLoader::singleton().add_resource_format_loader(&loader);
        
        Self { base, loader, saver }
    }
}
```

```admonish warning title="at_front behavior"
The ordering of `at_front` may currently not work as expected in Godot. For more information, see PR [godot#101543] or
book discussion [#65][book#65].
```


The minimal code for a **saver**, with all required virtual methods defined:

```rust
#[derive(GodotClass)]
#[class(base=ResourceFormatSaver, init, tool)]
struct MyAssetSaver {
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl IResourceFormatSaver for MyAssetSaver {
    // If you want a custom extension name (e.g., resource.myextension), 
    // then override this.
    fn get_recognized_extensions(
        &self,
        res: Option<Gd<Resource>>
    ) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        
        // Even though the Godot docs state that you don't need this check, it is
        // in fact necessary.
        if Self::is_recognized_resource(res) {
            // It is also possible to add multiple extensions per Saver.
            array.push("myextension");
        }
        
        array
    }

    // All resource types that this saver should handle must return true.
    fn is_recognized_resource(res: Option<Gd<Resource>>) -> bool {
        // It is also possible to add multible resource types per Saver.
        res.expect("Godot called this without an input resource?")
            .is_class("MyResourceType")
    }

    // This defines your logic for actually saving your resource.
    fn save(
        &mut self,
        // The resource that is currently getting saved.
        resource: Option<Gd<Resource>>,
        // The path that the resource is getting saved at.
        path: GString,
        // Flags for saving (see link below).
        flags: u32,
    ) -> godot::global::Error {
        // TODO: Put your saving logic in here, with the `GFile` API (see link below).
        
        godot::global::Error::OK
    }
}
```

Here are direct doc links to `SaverFlags` ([Godot][godot-saverflags], [Rust][api-saverflags]) and [`GFile`][api-gfile].


The minimal code for a **loader**, with all required virtual methods defined:

```rust
#[derive(GodotClass)]
#[class(init, tool, base=ResourceFormatLoader)]
struct MyAssetLoader {
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl IResourceFormatLoader for MyAssetLoader {
    // All file extensions you want to be redirected to your loader 
    // should be added here.
    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut arr = PackedStringArray::new();
        arr.push("myextension");
        arr
    }

    // All resource types that this loader handles.
    fn handles_type(&self, ty: StringName) -> bool {
        ty == "MyResourceType".into()
    }

    // The stringified name of your resource should be returned.
    fn get_resource_type(&self, path: GString) -> GString {
        // The extension arg always comes with a `.` in Godot, so don't forget it ;)
        if path.get_extension().to_lower() == ".myextension".into() {
            "MyResourceType".into()
        } else {
            // In case of not handling the given resource, this function must
            // return an empty string.
            GString::new()
        }
    }

    // The actual loading and parsing of your data.
    fn load(
        &self,
        // The path that should be openend to load the resource.
        path: GString,
        // If the resource was part of a import step you can access the original file
        // with this. Otherwise this path is equal to the normal path.
        original_path: GString,
        // This parameter is true when the resource is loaded with
        // load_threaded_request(). 
        // Internal implementations in Godot also ignore this parameter.
        _use_sub_threads: bool,
        // If you want to provide custom caching this parameter is the CacheMode enum.
        // You can look into the ResourceLoader docs to learn about the values.
        // When calling the default load() method, cache_mode is CacheMode::REUSE.
        cache_mode: i32,
    ) -> Variant {
        // TODO: Put your saving logic in here, with the `GFile` API (see link below).

        // If your loading operation failed and you want to handle errors,
        // you can return a godot::global::Error and cast it to a Variant.
    }
}
```

Direct link to `CacheMode` ([Godot][godot-cachemode], [Rust][api-cachemode]) and [`GFile`][api-gfile].

[godot-cachemode]: https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html#enum-resourceformatloader-cachemode
[api-cachemode]: https://godot-rust.github.io/docs/gdext/master/godot/classes/resource_loader/enum.CacheMode.html

[godot-saverflags]: https://docs.godotengine.org/en/stable/classes/class_resourcesaver.html#enum-resourcesaver-saverflags
[api-saverflags]: https://godot-rust.github.io/docs/gdext/master/godot/classes/resource_saver/struct.SaverFlags.html
[api-gfile]: https://godot-rust.github.io/docs/gdext/master/godot/prelude/struct.GFile.html

[godot-saver]: https://docs.godotengine.org/en/stable/classes/class_resourceformatsaver.html
[godot-loader]: https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html

[godot#101543]: https://github.com/godotengine/godot/pull/101543
[book#65]: https://github.com/godot-rust/book/pull/65#issuecomment-2585403123
