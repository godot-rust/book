<!--
  ~ Copyright (c) godot-rust; Bromeon and contributors.
  ~ This Source Code Form is subject to the terms of the Mozilla Public
  ~ License, v. 2.0. If a copy of the MPL was not distributed with this
  ~ file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

# Contributing to godot-rust

This chapter provides deeper information for people who are interested in contributing to the library.
In case you are simply _using_ godot-rust, you can skip this chapter.

If you haven't already, please read the [Contributing guidelines] in the repository first.
The rest of this chapter explains developer tools and workflows in more detail. Check out the respective subchapters.

```admonish tip
We highly appreciate if contributors propose a rough design before spending large effort on implementation.
This aligns ideas early and saves time on approaches that may not work.
```


## GitHub issues

We use [issues on GitHub] as a project management tool, not as a loose collection of ideas. To keep the issue tracker useful for both
contributors and maintainers, we have established some rules.

Before opening a new issue, please search existing ones (including closed) and link them if related. Make sure you are using the latest `master`
branch -- run `cargo update` if needed.

Every issue should be **actionable**, meaning:

1. It is **reproducible**. We don't expect that reporters go deep into debugging, but we expect a minimal, self-contained example that
   demonstrates a problem. This is true for both bugs and features (in which case the problem is a current limitation of the API).

2. It has a **well-defined scope**. It's clear under which conditions it is resolved.
   - "Improve error messages" or "my extension is slow" isn't helpful, but pointing to concrete messages or performance bottlenecks is.

3. It is fundamentally **solvable**. Given reasonable time, a motivated contributor could pick it up and make meaningful progress.
   Of course, some cases require significant knowledge of inner workings, but this is grounded in a technical challenge, not a formal one.
   - We don't track issues outside our control, unless there's a realistic chance to fix something in the near future (for which we have
     the `upstream` label). For example, potential Rust features 5 years into the future or big Godot changes that haven't even entered
     discussion stage fall into this category.

We appreciate reporters being available in the initial phase for follow-up clarifications, for example to narrow down when a bug occurs, or to
understand the use case behind a suggested change. In rare cases, issues may be closed as `status: wontfix` if they deliver too low return on
investment. Important here is not just initial development effort, but ongoing maintenance cost -- which no longer falls on the original
contributor, and is thus often overlooked.

godot-rust's approach may differ from other open-source projects. However, we believe that having a huge collection of vague requests doesn't do
anyone a favor. It makes it hard to see which issues are truly relevant and planned to be addressed; it discourages contributors from opening
new issues (knowing those might just get buried); and it creates extra triage work for maintainers.

Sometimes people use issues to ask questions. This is fine -- we use the `question` label for those and usually close them after an answer.

[Contributing guidelines]: https://github.com/godot-rust/gdext/blob/master/Contributing.md
[issues on GitHub]: https://github.com/godot-rust/gdext/issues?q=is%3Aissue%20state%3Aopen%20sort%3Aupdated-desc
