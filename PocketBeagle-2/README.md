# PocketBeagle 2 Examples

This folder provides multi-language examples for PocketBeagle 2. The primary language for examples will be [Python](https://www.python.org/). Examples will be present in other languages such as [Rust](https://www.rust-lang.org/) on best effort basis.

1. [blinky](blinky): Simple example to toggle GPIOs
2. [button](button): Simple example to detect button presses.
3. [fade](fade): Simple example to fade an LED in and out using PWM.
4. [eeprom](eeprom): Read and parse EEPROM contents.
5. [light_sensor](light_sensor): Simple example logging light sensor data.
6. [seven_segment](seven_segment): Simple example to demonstrate 2 seven segments on TechLab Cape.
7. [tonal_buzzer](tonal_buzzer): Simple example to play Harry Potter meoldy on a buzzer.
8. [rgb_led](rgb_led): Simple example using RGB Led using the multicolor LED Linux driver.

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
