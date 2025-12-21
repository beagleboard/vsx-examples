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

# Helpful Links

- [Typst Docs](https://typst.app/docs)
