use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn makeOutput(msg: String) -> std::io::Result<()> {
    let mut f = F {
        file: OpenOptions::new()
            .append(true)
            .create(true)
            .open("./output_euishi.txt")?,
    };

    f.write_to_file(format!(
        "Time : {}\n\
        {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    ));
    _ = f.file.flush();

    Ok(())
}

struct F {
    file: File,
}
impl F {
    fn write_to_file<T: AsRef<str>>(&mut self, msg: T) {
        _ = self.file.write_all(msg.as_ref().as_bytes());
    }
}
