# PocketBeagle 2 Examples

This folder provides multi-language examples for PocketBeagle 2. The primary language for examples will be [Python](https://www.python.org/). Examples will be present in other languages such as [Rust](https://www.rust-lang.org/) on best effort basis.

1. [blinky](blinky): Simple example to toggle GPIOs

# Initial Prep

Add python library folder to the path. Not sure if there is any other elegant way to do this:

```console
export PYTHONPATH=$HOME/vsx-examples/Libraries/python/beagle_linux_sdk/src:$PYTHONPATH
```

# Run Examples

## Python

```console
python {example_name}/python/main.py
```

## Rust

```console
cd {example_name}/rust
cargo run
```
