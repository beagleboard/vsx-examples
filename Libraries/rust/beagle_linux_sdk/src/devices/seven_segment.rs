use std::{
    fs::{File, OpenOptions},
    io::{Seek, Write},
    path::Path,
    time::Duration,
};

pub struct SevenSegment {
    scroll_step_file: File,
    message_file: File,
}

impl SevenSegment {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let base_dir = Path::new(path);

        if !base_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotADirectory,
                "Base path should be directory",
            ));
        }

        let scroll_step_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(base_dir.join("scroll_step_ms"))?;
        let message_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(base_dir.join("message"))?;

        Ok(Self {
            scroll_step_file,
            message_file,
        })
    }

    pub fn set_message(&mut self, msg: &str) -> std::io::Result<()> {
        let _ = self.message_file.seek(std::io::SeekFrom::Start(0));
        self.message_file.write_all(msg.as_bytes())?;
        Ok(())
    }

    pub fn set_step(&mut self, step: Duration) -> std::io::Result<()> {
        let _ = self.scroll_step_file.seek(std::io::SeekFrom::Start(0));
        self.scroll_step_file
            .write_all(step.as_millis().to_string().as_bytes())?;
        Ok(())
    }
}
