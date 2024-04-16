<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering constants

Constants can be used to share fixed values from Rust code to the Godot engine.

See also [GDScript reference for constants][godot-gdscript-constants].


## Constant declaration

Constants are declared as `const` items in Rust, inside the inherent `impl` block of a class.

The attribute `#[constant]` makes it available to Godot.

```rust
#[godot_api]
impl Monster {
    #[constant]
    const DEFAULT_HP: i32 = 100;

    #[func]
    fn from_name_hp(name: GString, hitpoints: i32) -> Gd<Self> { ... }
}
```

Usage in GDScript would look as follows:

```php
var nom = Monster.from_name_hp("Nomster", Monster.DEFAULT_HP)
var orc = Monster.from_name_hp("Orc", 200)
```

(This particular example might be better suited for default parameters once they are implemented, but it illustrates the point.)


## Statics

`static` fields can currently not be registered as constants.


[godot-gdscript-constants]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#constants
