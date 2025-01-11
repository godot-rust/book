<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# ResourceFormatSaver/Loader

The [`ResourceFormatSaver`] and [`ResourceFormatLoader`] classes allow you to serialize and deserialize your godot-rust resource-derived
classes with a custom procedure, as well as define new recognized file extensions. This is mostly useful if you have resources that contain
_pure Rust state_. "Pure" in this context refers to members of your struct that don’t have any `#[var]` or similar annotations, i.e. Godot
isn't aware of them. This can easily be the case when you work with Rust libraries.

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
        }
    }
}
```

Second, copy and paste this wherever you want in your library, deleting everything not necessary for your use case.

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
        ResourceSaver::singleton().add_resource_format_saver(&saver);
        ResourceLoader::singleton().add_resource_format_loader(&loader);
        
        Self { base, loader, saver }
    }
}

#[derive(GodotClass)]
#[class(base=ResourceFormatSaver, init, tool)]
struct MyAssetSaver {
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl IResourceFormatSaver for MyAssetSaver {
    // If you want a custom extension name (e.g., resource.myextension), then override this.
    fn get_recognized_extensions(&self, res: Option<Gd<Resource>>) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        
        // Even though the Godot docs imply you don't need this check, it is in fact necessary.
        if Self::is_recognized_resource(res) {
            array.push("myextension");
        }
        
        array
    }

    // All resource types that this saver should handle must return true.
    fn is_recognized_resource(res: Option<Gd<Resource>>) -> bool {
        res.expect("Godot called this without an input resource?")
            .is_class("MyResourceType")
    }

    // This defines your logic for actually saving your resource.
    fn save(
        &mut self,
        resource: Option<Gd<Resource>>,
        path: GString,
        flags: u32,
    ) -> godot::global::Error {
        // TODO: Put your saving logic in here, with the `FileAccess` API.
        
        godot::global::Error::OK
    }
}

#[derive(GodotClass)]
#[class(init, tool, base=ResourceFormatLoader)]
struct MyAssetLoader {
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl IResourceFormatLoader for MyAssetLoader {
    // All file extensions you want to be redirected to your loader should be added here.
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
        // This is a slight hack to check if the file has the right extension.
        // You can change this to your heart's content.
        if path.to_string().ends_with(".myextension") {
            "MyResourceType".into()
        } else {
            // In case of not handling the given resource, it must return an empty string.
            GString::new()
        }
    }

    // The actual loading and parsing of your data.
    fn load(
        &self,
        path: GString,
        original_path: GString,
        use_sub_threads: bool,
        cache_mode: i32,
    ) -> Variant {
        // TODO: Please put your loading logic in here.
    }
}
```

```admonish note title="The need for a singleton"
 Technically, the singleton is not strictly necessary -- Godot will keep the references around, and on exit, `ClassDB` will clean up for you
  -- thus not leaking memory. However, this approach is cleaner, and the performance cost of one singleton is negligible.
```

[`ResourceFormatSaver`]: https://docs.godotengine.org/en/stable/classes/class_resourceformatsaver.html
[`ResourceFormatLoader`]: https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html
