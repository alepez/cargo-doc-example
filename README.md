This is an empty crate. I'm just using this repo to play with `cargo`, *GitHub*
workflows and generation of *GitHub Pages*.

- [This is an internal link to a md file](doc/inner-page.md) `doc/inner-page.md`
- [This is an internal link to a md file](/doc/inner-page.md) `/doc/inner-page.md`
- [This is an external website](https://alepez.dev) `https://alepez.dev`
- [This is an internal link to a md file][relative-inner-page]
- [This is an internal link to a md file][absolute-inner-page]
- [This is an external website][external-website]

[relative-inner-page]: doc/inner-page.md
[absolute-inner-page]: /doc/inner-page.md
[external-website]: https://alepez.dev

## Enable GitHub Pages

This repo has *GitHub Pages* enabled. If you are cloning this repo, you need
to enable it, otherwise the *doc* workflow will fail.

To set up *GitHub Pages*:

- Settings
- Pages
- Set "Source" to "GitHub Actions"
