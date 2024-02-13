<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Philosophy

Different gamedev projects have different goals, which determines how APIs are built and how they support various use cases.

Understanding the vision behind gdext allows users to:

- decide whether the library is the right choice for them
- comprehend design decisions that have influenced the library's status quo
- contribute in ways that align with the project, thus saving time.


## Mission statement

If the idea behind the godot-rust project had to be summarized in a single word, it would be:

```admonish tip title="Pragmatism"
**godot-rust** offers an ergonomic, safe and efficient way to access Godot functionality from Rust.

It focuses on a productive workflow for the development of games and interactive applications.
```

In our case, pragmatism means that progress is driven by solutions to real-world problems, rather than theoretical purity.
Engineering comes with trade-offs, and gdext in particular is rather atypical for a Rust project. As such, we may sometimes deviate
from Rust best practices that may apply in a clean-room setting, but fall apart when exposed to the interaction with a C++ game engine.

At the end of the day, people use Godot and Rust to build games, simulations or other interactive applications. The library should be designed
around this fact, and Rust should be a tool that helps us achieve this goal -- not an end in itself.

In many ways, we follow [similar principles as the Godot engine][godot-contributor-best-practices].


## Scope

gdext is primarily a _binding_ to the Godot engine. A priority is to make Godot functionality accessible for Rust developers, in ways
that exploit the strengths of the language, while minimizing the friction.

Since we are not building our own game engine, features need to be related to Godot. We aim to build a robust core for everyday workflows,
while avoiding overly niche features. Integrations with other parts of the gamedev ecosystem (e.g. ECS, asset pipelines, GUI) are out of
scope and best implemented as extensions.


## API design principles

We envision the following core principles as a guideline for API design:

1. **Solution-oriented approach**  
   Every feature must solve a concrete problem that users or developers face.
   - We do not build solutions in search of problems. "Idiomatic Rust", "others also do it" or "it would be nice" are not good justifications :)
   - Priority is higher if more people are affected by a problem, or if the problem impacts a daily workflow more severely. In particular, this
     means that we can't spend much time on rarely used niche APIs, while there are game-breaking bugs in the core functionality.
   - We should always keep the big picture in mind. Rust makes it easy to get lost in irrelevant details. What matters is how a certain change
     helps end users.

2. **Simplicity**  
   Prefer self-explanatory, straightforward APIs.
   - Avoid abstractions that don't add value to the user.
     Do not over-engineer prematurely just because it's possible; follow [YAGNI][wiki-yagni] and avoid [premature optimization][wiki-premature-opt].
   - Examples to avoid: traits that are not used polymorphically, type-state pattern, many generic parameters,
     layers of wrapper types/functions that simply delegate logic.
   - Sometimes, runtime errors are better than compile-time errors. Most users are building a game, where fast iteration is key.
     Use `Option`/`Result` when errors are recoverable, and panics when the user must fix their code.
     See also [Ergonomics and panics][lib-ergonomics-panics].

3. **Maintainability**  
   Every line of code added **must be maintained, potentially indefinitely**.
   - Consider that it may not be you working with it in the future, but another contributor or maintainer, maybe a year from now.
   - Try to see the bigger picture -- how important is a specific feature in the overall library? How much detail is necessary?
     Balance the amount of code with its real-world impact for users.
   - Document non-trivial thought processes and design choices as inline `//` comments.
   - Document behavior, invariants and limitations in `///` doc comments.

4. **Consistency**  
   As a user, having a uniform experience when using different parts of the library is important.
   This reduces the cognitive load of learning and using the library, requires less doc lookup and makes users more efficient.
   - Look at existing code and try to understand its patterns and conventions.
   - Before doing larger refactorings or changes of existing systems, get an understanding of the underlying design choices
     and discuss your plans.

See these as guidelines, not hard rules. If you are unsure, please don't hesitate to ask questions and discuss different ideas :)

```admonish tip
We highly appreciate if contributors propose a rough design before spending large effort on implementation.
This aligns ideas early and saves time on approaches that may not work.
```

[wiki-premature-opt]: https://en.wikipedia.org/wiki/Program_optimization#When_to_optimize
[wiki-yagni]: https://en.wikipedia.org/wiki/YAGNI
[lib-ergonomics-panics]: https://godot-rust.github.io/docs/gdext/master/godot/#ergonomics-and-panics
[godot-contributor-best-practices]: https://docs.godotengine.org/en/stable/contributing/development/best_practices_for_engine_contributors.html
