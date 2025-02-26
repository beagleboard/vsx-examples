# Light Sensor

Simple example logging light sensor data. Uses `P1.19` which is connected to Light Sensor on TechLab cape.

# Usage

## Python

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle2/light_sensor/python$ python main.py
Light
Light
Light
Light
Dark
Dark
Dark
Dark
```

## Rust

```console
debian@pocketbeagle2:~/vsx-examples/PocketBeagle2/light_sensor/rust$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/rust`
Value: true
Value: true
Value: true
Value: false
Value: false
```
