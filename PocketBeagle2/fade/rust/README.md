# Fade

Simple example to fade an LED in and out using PWM. Also contains abstractions to use PWM using sysfs.

Uses `P1.36`. In case of [TechLab Cape](https://www.beagleboard.org/boards/techlab), this PIN is connected to `RGB.B`

# Usage

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle2$ cargo run -p fade -r
   Compiling fade v0.1.0 (/home/debian/vsx-examples/PocketBeagle2/fade)
    Finished `release` profile [optimized] target(s) in 3.85s
     Running `target/release/fade`
```

# Helpful resources

- [Linux Kernel PWM sysfs](https://docs.kernel.org/driver-api/pwm.html#using-pwms-with-the-sysfs-interface)
