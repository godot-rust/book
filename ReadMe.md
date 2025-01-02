# The godot-rust book

The godot-rust book is a user guide for **gdext**, the Rust bindings to Godot 4.
It covers a large part of the concepts and complements [the API docs][gdext-docs].
There is also [gdnative-book] for Godot 3.

> [!Tip]
> The book is deployed at **[godot-rust.github.io/book][book-web]**.


## Local setup

The book is built with [mdBook] and the plugins [mdbook-toc] and [mdbook-admonish]. To install them and build the book locally, you can run:

```bash
cargo install mdbook mdbook-toc mdbook-admonish
mdbook build
```

To run a local server with automatic updates while editing the book, use:

```bash
mdbook serve --open
```


### Formatting and linting

[markdownlint] enforces a consistent style across the Markdown files.
It is automatically run during CI, but if you have the `npm` toolchain, you can also run it locally:

```bash
npm install --global markdownlint-cli2
./lint.sh

# To fix certain errors directly:
./lint.sh fix
```


### Oxipng

We use [oxipng] to optimize image file size.
You can install it with `cargo install oxipng` and then run it as follows:

```bash
oxipng --strip safe --alpha -r src
```


## Contributing

This repository is for documentation only. For changes in the library itself, please open pull requests and issues in the [main repo][gdext],
and read the [contributing guidelines][gdext-contribute].


## License

Like gdext itself, the gdext book is licensed under [MPL 2.0][mpl].

[book-web]: https://godot-rust.github.io/book
[gdext]: https://github.com/godot-rust/gdext
[gdext-docs]: https://godot-rust.github.io/docs/gdext/master/godot
[gdext-contribute]: https://github.com/godot-rust/gdext/blob/master/Contributing.md
[gdnative-book]: https://github.com/godot-rust/gdnative-book
[markdownlint]: https://github.com/DavidAnson/markdownlint
[mdbook-admonish]: https://github.com/tommilligan/mdbook-admonish
[mdbook-toc]: https://github.com/badboy/mdbook-toc
[mdBook]: https://github.com/rust-lang-nursery/mdBook
[mpl]: https://www.mozilla.org/en-US/MPL
[oxipng]: https://github.com/shssoichiro/oxipng

