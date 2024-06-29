<!--
 ~ Copyright (c) godot-rust; Bromeon and contributors.
 ~ This Source Code Form is subject to the terms of the Mozilla Public
 ~ License, v. 2.0. If a copy of the MPL was not distributed with this
 ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Export to Android

Exporting with gdext for Godot requires some of the same pieces that are required for building Godot from source.
Specifically, the Android SDK Command Line Tools and JDK 17 as mentioned in Godot's documentation
[here](https://docs.godotengine.org/en/stable/contributing/development/compiling/compiling_for_android.html#requirements).

Once you have those installed, you then need to follow Godot's instructions for setting up the build system
[here](https://docs.godotengine.org/en/stable/contributing/development/compiling/compiling_for_android.html#setting-up-the-buildsystem).

To find the jdk and nkd versions that are needed, reference the Godot configuration that your version of Godot is using.  For example:

- [master branch](https://github.com/godotengine/godot/blob/master/platform/android/java/app/config.gradle)
- [4.2.2-stable tag](https://github.com/godotengine/godot/blob/4.2.2-stable/platform/android/java/app/config.gradle)


## Compiling

The environment variable `CLANG_PATH` is used by bindgen's clang-sys dependency. See also
[clang-sys documentation](https://github.com/KyleMayes/clang-sys?tab=readme-ov-file#environment-variables)

Set the environment variable `CLANG_PATH` to point to Android's build of clang. Example:

```bash
export CLANG_PATH=\
"{androidCliDirectory}/{androidCliVersion}/ndk/{ndkVersion}/toolchains/llvm/prebuilt/{hostMachineOs}/bin/clang"
```

Then set the `CARGO_TARGET_{shoutTargetTriple}_LINKER` to point to the Android linker for the Android triple you are targeting.
The `{shoutTargetTriple}` should be in `SHOUT_CASE` so that a triple such as `aarch64-linux-android` becomes `AARCH64_LINUX_ANDROID`.
You need to compile your gdext library for each Android triple individually. Possible targets can be found by running:

```bash
rustup target list
```

You can find the linkers in the Android CLI directory at:

```text
{androidCliDirectory}/{androidCliVersion}/ndk/{ndkVersion}/toolchains/llvm/prebuilt/
{hostMachineOs}/bin/{targetTriple}{androidVersion}
```

As of writing this, the tested triples are:

| Triple                      | Environment Variable                            | Godot Arch     | GDExtension Config            |
| --------------------------- | ----------------------------------------------- | -------------- | ----------------------------- |
| `aarch64-linux-android`     | `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER`     | `arm64`        | `android.debug.arm64`         |
| `x86_64-linux-android`      | `CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER`      | `x86_64`       | `android.debug.x86_64`        |
| `armv7-linux-androideabi`   | `CARGO_TARGET_ARMV7_LINUX_ANDROID_LINKER`       | `arm32`        | `android.debug.armeabi-v7a`   |
| `i686-linux-android`        | `CARGO_TARGET_I686_LINUX_ANDROID_LINKER`        | `x86_32`       | `android.debug.x86`           |

Notice how the environment variables are in all-caps and the triple's "-" is replaced with "_".

Make sure to add all of the triples you want to support to `rustup` via:

```bash
rustup target add {targetTriple}
```

Example:

```bash
rustup target add aarch64-linux-android
```


## A complete example

Putting it all together, here is an example compiling for `aarch64-linux-android`. This is also probably the most common
Android target, as of the writing of this.

Assuming the following things:

1. Android CLI is installed in the `$HOME` folder.
2. Godot is still relying on Android NDK version 23.2.8568313. Check
[here](https://github.com/godotengine/godot/blob/master/platform/android/java/app/config.gradle).
3. The downloaded Android CLI version is: 11076708_latest (update this to be the version you downloaded).
4. This is being run on Linux. Change the `linux-x86_64` folder in `CLANG_PATH` and `CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER`
to be your host machine's operating system.
5. You are targeting Android version 34.

And here is what the commands look like running from a bash shell:

```bash
rustup target add aarch64-linux-android

export CLANG_PATH="$HOME/android-cli/11076708_latest/ndk/23.2.8568313/toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=\
"$HOME/android-cli/11076708_latest/ndk/23.2.8568313/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

cargo build --target=aarch64-linux-android
```

And then you should find a built version of your GDExtension library in:

```text
target/aarch64-linux-android/debug/{YourCrate}.so
```

Make sure to update your `.gdextension` file to point to the compiled lib. Example:

```text
android.debug.arm64="res://path/to/rust/lib/target/aarch64-linux-android/debug/{YourCrate}.so
```
