from pathlib import Path

BASE_SYSFS_PATH = "/sys/class/pwm/pwmchip"


class PwmChannel:
    def __init__(self, chip_dir: Path, channel: int) -> None:
        self.chip_dir = chip_dir
        self.channel = channel
        
        chan_dir = self.chip_dir.joinpath(f"pwm{channel}")
        if not chan_dir.is_dir():
            raise ValueError("PWM channel does not exist")

        self.enable_p = chan_dir.joinpath("enable")
        self.duty_cycle = chan_dir.joinpath("duty_cycle")
        self.period = chan_dir.joinpath("period")

    def enable(self):
        with open(self.enable_p, 'w') as f:
            f.write("1")

    def set_duty_cycle(self, duty: int):
        with open(self.duty_cycle, 'w') as f:
            f.write(str(duty))

    def set_period(self, period: int):
        with open(self.period, 'w') as f:
            f.write(str(period))

    def __del__(self):
        unexport = self.chip_dir.joinpath("unexport")
        with open(unexport, 'w') as f:
            f.write(str(self.channel))


class PwmChip:
    def __init__(self, n: int) -> None:
        self.chip_dir = Path(f"{BASE_SYSFS_PATH}{n}")
        self.export_p = self.chip_dir.joinpath("export")
        self.unexport_p = self.chip_dir.joinpath("unexport")

        if not self.chip_dir.is_dir():
            raise ValueError("PWM Chip does not exist")

    def unexport(self, channel: int):
        with open(self.unexport_p, 'w') as f:
            f.write(str(channel))

    def export(self, channel: int) -> PwmChannel:
        try:
            self.unexport(channel)
        except Exception as e:
            pass

        with open(self.export_p, 'w') as f:
            f.write(str(channel))

        return PwmChannel(self.chip_dir, channel)
