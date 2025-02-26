from pathlib import Path


class SevenSegment:
    def __init__(self, p: Path) -> None:
        if not p.is_dir():
            raise ValueError("Seven segment path does not exist")

        self.scroll_step_path = p.joinpath("scroll_step_ms")
        self.message_path = p.joinpath("message")

    def set_message(self, msg: str):
        with open(self.message_path, 'w') as f:
            f.write(msg)

    def set_step(self, step_ms: int):
        with open(self.scroll_step_path, 'w') as f:
            f.write(str(step_ms))
