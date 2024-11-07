<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Registering signals

Signals currently have very limited support in gdext, through the `#[signal]` attribute. Consult its [API documentation][api-signal] for details.

Signal registration will be completely reworked in the future, with breaking API changes.

As an alternative, you can use Godot's dynamic API to register signals. The [`Object` class][api-object] has methods `connect()` and
`emit_signal()` that can be used to connect and emit signals, respectively.

See also [GDScript reference for signals][godot-gdscript-signals].


[api-object]: https://godot-rust.github.io/docs/gdext/master/godot/classes/struct.Object.html
[api-signal]: https://godot-rust.github.io/docs/gdext/master/godot/register/derive.GodotClass.html#signals
[godot-gdscript-signals]: https://docs.godotengine.org/en/stable/tutorials/scripting/gdscript/gdscript_basics.html#signals
