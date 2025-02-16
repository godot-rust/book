# Notes about theme

Customized by adding two new themes `godot-rust-light` + `godot-rust-dark`.

Original files can be restored with `mdbook init --theme`, see also [Theme][mdbook-theme] on mdBook docs. 
This should be done from time to time to keep up with mdBook updates in their CSS. Changes specific to this book
should then be cherry-picked.

Following CSS files exist:
- `theme/css/chrome.css`, `print.css` -- no changes from defaults needed, thus omitted.
- `theme/css/general.css` -- change from default is overridden in `variables.css` via `!important`, thus also omitted.
- `theme/css/variables.css` -- contains all color definitions, should be updated often.
- `theme/css/highlight.css` -- syntax highlighting colors.
- `theme/css/tomorrow-night.css` -- syntax highlighting in dark mode.
- `config/mdbook-admonish.css` -- for admonish plugin; needs customization to look decent in dark mode.

Generally, CSS changes should be applied automatically while `mdbook serve` is active; however some (like admonish)
need a re-launch of `mdbook serve` to take effect.

[mdbook-theme]: https://rust-lang.github.io/mdBook/format/theme/index.html
