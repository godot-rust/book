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
`static` declarations cannot be used.

The attribute `#[constant]` makes the constant available to Godot.

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


## Limitations

Godot supports **only integers** to be registered as constants via GDExtension API.

You can work around this by registering a static function, called as `Monster.DEFAULT_NAME()` in GDScript.

```rust
#[godot_api]
impl Monster {
    #[func(rename = "DEFAULT_NAME")]
    fn default_name() -> GString {
        "Monster_001".into()
    }
}
```

While you could technically use read-only properties, this is problematic because:

- You need an existing instance of the class.
- Every object occupies space for the constant.[^zst-properties]

It's really just syntax, an extra `()` will not derail your game. If you have a heavier value that you don't want to recompute (e.g. array),
you can always store it in a `thread_local!` in Rust.


[godot-gdscript-constants]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#constants
[issue-1151]: https://github.com/godot-rust/gdext/issues/1151

<br>

---

**Footnotes**

[^zst-properties]: In the future, we may have properties that don't occupy space, see [#1151][issue-1151].
