<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Custom node icons

By default, all your custom types will use the `Node` icon in the editor UI -- e.g. in the scene tree or when selecting a node to create.
While this can be serviceable, you may want to add custom icons to distinguish node types, especially if you plan to distribute your extension
to others.

All icons must be registered by their class name in your `.gdextension` file. For this, you can add a new `icon` section. Classes are keys and
paths to SVG files are values.

```toml
[icons]

MyClass = "res://addons/your_extension/filename.svg"
```

```admonish note title="Icon paths"
The path is based off the `res://` scheme, like other Godot resources. It is recommended to use Godot's convention of an `addons` folder,
followed by the name of the addon. 

Read more about the reasoning behind this in the Godot docs:
- [Installing plugins][godot-installing-plugins]
- [Making plugins][godot-making-plugins]
```

[godot-installing-plugins]: https://docs.godotengine.org/en/stable/tutorials/plugins/editor/installing_plugins.html#finding-plugins
[godot-making-plugins]: https://docs.godotengine.org/en/stable/tutorials/plugins/editor/making_plugins.html


## Formatting for custom icons

The Godot docs have a [page dedicated][godot-icons] to tools and resources for creating custom icons. The long and short of it is:

- Use the SVG format.
- Aspect ratio is a square, 16x16 units is the reference size.
- Refer to the [Godot icon colors mappings][gh-godot-colors].
  - Use the light mode colors -- Godot only supports light-to-dark, but not dark-to-light color conversions.

```admonish help "Third-party article"
The user _QueenOfSquiggles_ wrote an alternative version of this article [on her personal blog][qos-colors], which includes color previews for the
light and dark themed colors.

Details on how to use her reference page is included [here][qos-info].
```

[godot-icons]: https://docs.godotengine.org/en/stable/contributing/development/editor/creating_icons.html
[gh-godot-colors]: https://github.com/godotengine/godot/blob/master/editor/themes/editor_color_map.cpp
[qos-colors]: https://queenofsquiggles.github.io/tech/godot-icon-colours/
[qos-info]: https://queenofsquiggles.github.io/tech/godot-icon-colours/#how-to-use-this
