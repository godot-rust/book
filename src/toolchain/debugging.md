<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Debugging

Extensions written in godot-rust can be debugged using LLDB, in a similar manner to other Rust programs. The primary difference is that LLDB will
launch or attach to the Godot C++ executable: either the Godot editor or your custom Godot application.
Godot then loads your extension (itself a dynamic library), and with it your Rust code.

The process for launching or attaching LLDB varies based on your IDE and platform. Unless you are using a debug version of Godot itself,
you will only have symbols for stack frames in Rust code.


## Launching with VS Code

Here is an example debugger setup for Visual Studio Code. Launch configurations and tasks should be placed in`./.vscode/launch.json` and 
`./.vscode/tasks.json` respectively, relative to your project's root.

This example assumes you have the [CodeLLDB] extension installed, which is common for Rust development.

```jsonc
// ./.vscode/launch.json
{
    "configurations": [
        {
            "name": "Debug Project (Godot 4)",
            "type": "lldb",
            "request": "launch",
            "cwd": "${workspaceFolder}/godot",
            "preLaunchTask": "build-rust",
            "args": [
                "-w"
            ],
            "program": "PATH/TO/GODOT"
        }
    ]
}
```

```jsonc
// ./.vscode/tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "build-rust",
            "command": "cargo",
            "args": [
                "build"
            ],
            "options": {
                "cwd": "${workspaceFolder}/rust"
            }
        }
    ]
}
```


## Debugging on macOS

Attaching a debugger to an executable that wasn't compiled locally (the Godot editor, in this example) requires special considerations on macOS
due to its _System Integrity Protection_ (SIP) security feature. Even though your extension is compiled locally, LLDB will be unable to attach
to the Godot _host process_ without manual re-signing.

In order to re-sign, simply create a file called `editor.entitlements` with the following contents. Be sure to use the `editor.entitlements` file
below rather than the one from the [Godot Docs](https://docs.godotengine.org/en/stable/contributing/development/debugging/macos_debug.html),
as it includes the required `com.apple.security.get-task-allow` key not currently present in Godot's instructions.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist 
  PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>com.apple.security.cs.allow-dyld-environment-variables</key>
        <true/>
        <key>com.apple.security.cs.allow-jit</key>
        <true/>
        <key>com.apple.security.cs.allow-unsigned-executable-memory</key>
        <true/>
        <key>com.apple.security.cs.disable-executable-page-protection</key>
        <true/>
        <key>com.apple.security.cs.disable-library-validation</key>
        <true/>
        <key>com.apple.security.device.audio-input</key>
        <true/>
        <key>com.apple.security.device.camera</key>
        <true/>
        <key>com.apple.security.get-task-allow</key>
        <true/>
    </dict>
</plist>
```

Once this file is created, you can run

```bash
codesign -s - --deep --force --options=runtime \
    --entitlements ./editor.entitlements /Applications/Godot.app
```

in Terminal to complete the re-signing process. It is recommended to check this file into version control, since each developer needs to
re-sign their local installation if you have a team. This process should only be necessary once per Godot installation though.

[CodeLLDB]: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
