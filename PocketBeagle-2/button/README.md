# Button

Simple example to detect button presses. Uses Pin `P2.33` by default.

# Usage

## Python

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/button/python$ python main.py
Waiting for button press
Button Pressed
```

## Rust

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/button/rust$ cargo run
   Compiling button v0.1.0 (/home/debian/vsx-examples/PocketBeagle-2/button/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.22s
     Running `target/debug/button`
Waiting for button press
Button Pressed
```
