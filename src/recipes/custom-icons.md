<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Custom Icons

By default, all of your custom types will use the `Node` icon. While this can be serviceable, you will likely want to add custom
icons, especially if you plan to distribute this to others!

All of the icons must be registered by class name in your `.gdextension` file.

```toml
[icons]

MyClass = "res://addons/your_extension/file_name.svg"
```

```admonish hint
Note that the path is based off of the `res://` path, like all other resources for Godot.
This means that you will need to have a standard 
installation path for your tool. It is recommended to make this in the "addons" folder because 
this will clearly signify to users that it is a 
directory of third party resources and code and that they likely shouldn't mess 
with it unless they are certain of what they are doing.
```


## Formatting for custom icons

The Godot docs have a [page dedicated][gdocs-icons] to tools and resources for creating custom icons. The long and short of it is:

- make it an SVG
- make it use square aspect ratio and size (16x16 pixels is the referenced size)
- And refer to the [Godot icon colors mappings][ggit-colors].
  - Use the light mode colors, Godot only supports light to dark, and not dark to light color conversions.

```admonish help
QueenOfSquiggles has made a more readable version [on her personal blog][qos-colors] which includes color previews for the 
light and dark theme colors.

Details on how to use her reference page is included [here][qos-info].
```

[gdocs-icons]: https://docs.godotengine.org/en/stable/contributing/development/editor/creating_icons.html
[ggit-colors]:https://github.com/godotengine/godot/blob/master/editor/editor_themes.cpp#L62-L174
[qos-colors]: https://queenofsquiggles.github.io/tech/godot-icon-colours/
[qos-info]: https://queenofsquiggles.github.io/tech/godot-icon-colours/#how-to-use-this
