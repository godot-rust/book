# The godot-rust book

The godot-rust book is a user guide for **gdext**, the Rust bindings to Godot 4.
The book is still work-in-progress, and contributions are very welcome.

An online version of the book is available at [godot-rust.github.io/book][book-web].  
For the gdnative book, check out [gdnative-book].

The book is built with [mdBook] and the plugins [mdbook-toc] and [mdbook-admonish]. To install them and build the book locally, you can run:

```bash
cargo install mdbook mdbook-toc mdbook-admonish
mdbook build
```

To run a local server with automatic updates while editing the book, use:

```bash
mdbook serve --open
```


## Formatting and linting

We use [markdownlint] to enforce a consistent style across the Markdown files.
It is automatically run during CI, but if you have the `npm` toolchain, you can also run it locally:

```bash
npm install --global markdownlint-cli2
./lint.sh
```


## Oxipng

We use [oxipng](https://github.com/shssoichiro/oxipng) to optimize image file size.
You can install it with `cargo install oxipng` and then run it as follows:

```bash
oxipng --strip safe --alpha -r src


## Contributing

This repository is for documentation only. Please open pull requests targeting the gdext library itself in the [main repo][gdext].
Please read the corresponding contributing guidelines in `Contributing.md`.


## License

Like gdext itself, the gdext book is licensed under [MPL 2.0][mpl].

[book-web]: https://godot-rust.github.io/book
[gdext]: https://github.com/godot-rust/gdext
[gdnative-book]: https://github.com/godot-rust/gdnative-book
[markdownlint]: https://github.com/DavidAnson/markdownlint
[mdbook-admonish]: https://github.com/tommilligan/mdbook-admonish
[mdbook-toc]: https://github.com/badboy/mdbook-toc
[mdBook]: https://github.com/rust-lang-nursery/mdBook
[mpl]: https://www.mozilla.org/en-US/MPL
