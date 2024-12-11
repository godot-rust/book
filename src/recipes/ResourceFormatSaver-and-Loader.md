<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# ResourceFormatSaver/Loader
The [ResourceFormatSaver](https://docs.godotengine.org/en/stable/classes/class_resourceformatsaver.html) and [ResourceFormatLoader](https://docs.godotengine.org/en/stable/classes/class_resourceformatloader.html) classes allow you to serialize and deserialize your godot-rust resource derived classes with a custom procedure aswell as define new recognized file-extensions. This is mostly usefull if you have resources that contain Pure-Rust-State. Pure-Rust-State in this context are members of your struct that dont have any #[var] or similar annotations and godot doesnt know about. This can easily be the case when you work with rust libraries. The following content is intended to give you a copy/paste starting point while for advanced usecases the godot-docs of these classes should be consulted.

First of all you need to call the provided functions in your library entrypoint at the `InitLevel::Scene`.
This ensures proper initialization and freeing of your Loader/Saver.
```rust
/// Entrypoint into the library
#[gdextension]
unsafe impl ExtensionLibrary for YourGdExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            register_input_output();
        }
    }
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            unregister_input_output();
        }
    }
}
```
Second copy paste this whereever you want in your library deleting everything not necessary for your usecase.

```rust
//required imports
use godot::{
    classes::{
        Engine,  IResourceFormatLoader, IResourceFormatSaver,
        ResourceFormatLoader, ResourceFormatSaver, ResourceLoader, ResourceSaver,
    },
    prelude::*,
};


//Registering the Singleton with the Engine
pub fn register_input_output() {
    Engine::singleton().register_singleton(
        "YourInputOutputSingleton",
        &YourInputOutputSingleton::new_alloc(),
    );
}

//Unregisters the Singleton when the Engine exits
pub fn unregister_input_output() {
    Engine::singleton().unregister_singleton("YourInputOutputSingleton");
}

//The definition of the Singleton with all your loader/savers as members
//to keep the object references for destruction later
#[derive(GodotClass)]
#[class(tool, base=Object)]
struct YourInputOutputSingleton {
    base: Base<Object>,
    loader: Gd<YourAssetLoader>,
    saver: Gd<YourAssetSaver>,
}

#[godot_api]
impl IObject for YourInputOutputSingleton {
    fn init(base: Base<Object>) -> Self {
        let plugin = Self {
            base,
            loader: YourAssetLoader::new_gd(),
            saver: YourAssetSaver::new_gd(),
        };
        // Registering the Loader and Saver in Godot.
        ResourceSaver::singleton().add_resource_format_saver(&plugin.saver);
        ResourceLoader::singleton().add_resource_format_loader(&plugin.loader);
        plugin
    }
}


#[derive(GodotClass)]
#[class(init,tool, base=ResourceFormatSaver)]
struct YourAssetSaver {
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl IResourceFormatSaver for YourAssetSaver {
    //If you want a custom extension name e.g. resource.gdext override this 
    fn get_recognized_extensions(&self, res: Option<Gd<Resource>>) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        //Even though the godot docs imply you dont need this check it is in fact necessary
        if res.expect("Godot called this without a input resource?")
            .is_class("YourResourceType") {
                array.push("yourfileextensionname");
        }
        array
    }

    //All resource types that this Saver should handle must return true 
    fn recognize(&self, res: Option<Gd<Resource>>) -> bool {
        res.expect("Godot called this without a input resource?")
            .is_class("YourResourceType")
    }

    // This defines your logic for actually saving your Resource
    fn save(
        &mut self,
        resource: Option<Gd<Resource>>,
        path: GString,
        flags: u32,
    ) -> godot::global::Error {
        
        //TODO please put your saving logic in here with the FileAccess api

        godot::global::Error::OK
    }
}


#[derive(GodotClass)]
#[class(init,tool, base=ResourceFormatLoader)]
struct YourAssetLoader {
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl IResourceFormatLoader for YourAssetLoader {
    //All file extensions you want to be redirected to you loader should be added here
    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut arr = PackedStringArray::new();
        arr.push("yourfileextensionname");
        arr
    }

    //All resource types that this Loader handles
    fn handles_type(&self, typ: StringName) -> bool {
        typ == "YourResourceType".into()
    }

    //The stringified name of your Resource should be returned  
    fn get_resource_type(&self, path: GString) -> GString {
        //this is a slight hack to check if it has the right extension
        //you can change this to you hearts content
        if path.to_string().ends_with(".yourfileextensionname") {
            "YourResourceType".into()
        } else {
            //In case of not handling given resource it must return an empty string
            "".into()
        }
    }

    // The actual loading and parsing your data
    fn load(
        &self,
        path: GString,
        original_path: GString,
        use_sub_threads: bool,
        cache_mode: i32,
    ) -> Variant {
        //TODO please put your loading logic in here.
    }
}
```

Note:
- Technically the Singleton is not necessary as godot will keep the references around and on exit ClassDB will clean up for you - thus not leaking the memory - but this approch is cleaner and performance of one singleton is negligable.