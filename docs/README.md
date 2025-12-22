# Introduction

PocketBealge 2 TechLab hadount using [Typst](https://typst.app/).

# Build

Install Typst. I am going to install it using cargo. Other install instructions can be found [here](https://github.com/typst/typst?tab=readme-ov-file#installation).

```
cargo install --git https://github.com/typst/typst --locked typst-cli
```

Build PDF:

```
typst compile --font-path fonts main.typ
```

You will now see the generated PDF with the name `main.pdf`.

# Watch

Typst cli also supports watch functionality, that atches source files and recompiles on changes.

```
typst watch --font-path fonts main.typ
```

# Makefile

A simple Makefile is also provided with the following recipie: `watch` and `build`. The generated pdf is placed under `build/`.

# Special notes

The current layout requires special margin size for the chapter start pages. This is due to having special header and footer in these pages. Currently, there is no easy way to have different margin size based on some condition. So we need to manually change the margin from the 2nd page in a chapter using the following code:

```
#pagebreak()
#set page(margin: (x: bh.page_margin_x, y: 30pt))
```

Hopefully, Typst will support this at some point: https://github.com/typst/typst/issues/1483

# Helpful Links

- [Typst Docs](https://typst.app/docs)
