use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn makeOutput(msg: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // create if file doesn't exist
        .open("./output.txt")?;

    file.write_all("Time: ".as_bytes())?;
    file.write_all(getTime().as_bytes())?;
    file.write_all("\n".as_bytes())?;
    file.write_all(msg.as_bytes())?;

    file.flush()?;

    Ok(())
}

fn getTime() -> String {
    let local: DateTime<Local> = Local::now();

    format!("{}", local.format("%Y-%m-%d %H:%M:%S"))
}
