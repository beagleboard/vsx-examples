# Fade

Simple example to fade an LED in and out using PWM. Also contains abstractions to use PWM using sysfs.

Uses `P1.36`. In case of [TechLab Cape](https://www.beagleboard.org/boards/techlab), this PIN is connected to `RGB.B`

# Usage

## Python

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/python$ python main.py
```

## Rust

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle-2/rust$ cargo run
    Finished `release` profile [optimized] target(s) in 3.85s
     Running `target/release/rust`
```

# Helpful resources

- [Linux Kernel PWM sysfs](https://docs.kernel.org/driver-api/pwm.html#using-pwms-with-the-sysfs-interface)
