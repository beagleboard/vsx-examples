# Seven Segments Example

Simple example to demonstrate 2 seven segments on TechLab Cape. Needs root permissions.

# Usage

## Python

```console
root@pocketbeagle2:/home/debian/vsx-examples/PocketBeagle-2/seven_segment/python# python main.py
Countdown Automatic on Right
Countdown Manual on Left
```

## Rust

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/seven_segment/rust$ cargo build
   Compiling beagle_linux_sdk v0.1.0 (/home/debian/vsx-examples/Libraries/rust/beagle_linux_sdk)
   Compiling rust v0.1.0 (/home/debian/vsx-examples/PocketBeagle2/seven_segment/rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.27s
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/seven_segment/rust$ sudo ./target/debug/rust
Countdown Automatic
Countdown Manual
```
