<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Code and API conventions


<!-- toc -->

## Bikeshed auto-painting

In general, we try to automate as much as possible during CI. This ensures a consistent code style and avoids unnecessary work during
pull request reviews.

In particular, we use the following tools:

- [**rustfmt**] for code formatting ([config options][rustfmt-config]).
- [**clippy**] for lints and style warnings ([list of lints][clippy-lints]).
- Clang's [**AddressSanitizer**] and [**LeakSanitizer**] for memory safety.
- Various specialized tools:
  - [**skywalking-eyes**] to enforce license headers.
  - [**cargo-deny**] and [**cargo-machete**] for dependency verification.

In addition, we have unit tests (`#[test]`), doctests and Godot integration tests (`#[itest]`).
See [Dev tools] for more information.

[**AddressSanitizer**]: https://clang.llvm.org/docs/AddressSanitizer.html
[**cargo-deny**]: https://embarkstudios.github.io/cargo-deny
[**cargo-machete**]: https://github.com/bnjbvr/cargo-machete
[**clippy**]: https://doc.rust-lang.org/stable/clippy/usage.html
[**LeakSanitizer**]: https://clang.llvm.org/docs/LeakSanitizer.html
[**rustfmt**]: https://github.com/rust-lang/rustfmt
[**skywalking-eyes**]: https://github.com/apache/skywalking-eyes
[clippy-lints]: https://rust-lang.github.io/rust-clippy/master/index.html
[Dev tools]: dev-tools.md
[rustfmt-config]: https://rust-lang.github.io/rustfmt


## Technicalities

This section lists specific style conventions that have caused some confusion in the past.
Following them is nice for consistency, but it's not the top priority of this project. Hopefully, we can automate some of them over time.


### Formatting

`rustfmt` is the authority on formatting decisions. If there are good reasons to deviate from it, e.g. data-driven tables in tests,
use `#[rustfmt::skip]`. rustfmt does not work very well with macro invocations, but such code should still follow `rustfmt`'s
formatting choices where possible.

Line width is 120-145 characters (mostly relevant for comments).  
We use separators starting with  `// ---` to visually divide sections of related code.


### Code organization

1. Anything that is not intended to be accessible by the user, but must be `pub` for technical reasons, should be marked as `#[doc(hidden)]`.
   - This does [**not** constitute part of the public API][lib-public-api].

2. We do not use the `prelude` inside the project, except in examples and doctests.

3. Inside `impl` blocks, we _roughly_ try to follow the order:
   - Type aliases in traits (`type`)
   - Constants (`const`)
   - Constructors and associated functions
   - Public methods
   - Private methods (`pub(crate)`, private, `#[doc(hidden)]`)

4. Inside files, there is no strict order yet, except `use` and `mod` at the top. Prefer to declare public-facing symbols before private ones.

5. Use flat import statements. If multiple paths have different prefixes, put them on separate lines. Avoid `self`.
   ```rust
   // Good:
   use crate::module;
   use crate::module::{Type, function};
   use crate::module::nested::{Trait, some_macro};
   
   // Bad:
   use crate::module::{self, Type, function, nested::{Trait, some_macro}};
   ```


### Types

1. Avoid tuple-enums `enum E { Var(u32, u32) }` and tuple-structs `struct S(u32, u32)` with more than 1 field. Use named fields instead.

2. Derive order is `#[derive(GdextTrait, ExternTrait, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]`.
   - `GdextTrait` is a custom derive defined by godot-rust itself (in any of the crates).
   - `ExternTrait` is a custom derive by a third-party crate, e.g. `nanoserde`.
   - The standard traits follow order _construction, comparison, hashing, debug display_.
     More expressive ones (`Copy`, `Eq`) precede their implied counterparts (`Clone`, `PartialEq`).


### Functions

1. Getters don't have a `get_` prefix.

2. Use `self` instead of `&self` for `Copy` types, unless they are really big (such as `Transform3D`).

3. For `Copy` types, avoid in-place mutation `vector.normalize()`.  
   Instead, use `vector = vector.normalized()`. The past tense indicates a copy.

4. Annotate with `#[must_use]` when ignoring the return value is likely an error.  
   Example: builder APIs.


### Attributes

Concerns both `#[proc_macro_attribute]` and the attributes attached to a `#[proc_macro_derive]`.

1. Attributes always have the same syntax: `#[attr(key = "value", key2, key_three = 20)]`
   - `attr` is the outer name grouping different key-value pairs in parentheses.  
     A symbol can have multiple attributes, but they cannot share the same name.
   - `key = value` is a key-value pair. just `key` is a key-value pair without a value.
     - Keys are always `snake_case` identifiers.  
     - Values are typically strings or numbers, but can be more complex expressions.
     - Multiple key-value pairs are separated by commas. Trailing commas are allowed.

2. In particular, avoid these forms:
   - `#[attr = "value"]` (top-level assignment)
   - `#[attr("value")]` (no key -- note that `#[attr(key)]` is allowed)
   - `#[attr(key(value))]`
   - `#[attr(key = value, key = value)]` (repeated keys)

The reason for this choice is that each attribute maps nicely to a map, where values can have different types.
This allows for a recognizable and consistent syntax across all proc-macro APIs. Implementation-wise, this pattern is
directly supported by the `KvParser` type in godot-rust, which makes it easy to parse and interpret attributes.


[lib-public-api]: https://godot-rust.github.io/docs/gdext/master/godot/#public-api
