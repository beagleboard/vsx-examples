from time import sleep
from sysfs import Device

DEV_NAME = "mma8453"

accel = Device(name=DEV_NAME)

scale = accel.sysfs("in_accel_scale").read_float()
x_raw = accel.sysfs("in_accel_x_raw")
y_raw = accel.sysfs("in_accel_y_raw")
z_raw = accel.sysfs("in_accel_z_raw")

while True:
    x_scaled = x_raw.read_float() * scale
    y_scaled = y_raw.read_float() * scale
    z_scaled = z_raw.read_float() * scale
    print(
        f"Acceleration along X = {x_scaled:.2f} ms^2, Y = {y_scaled:.2f} ms^2, Z = {z_scaled:.2f} ms^2"
    )
    sleep(1)
