use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn makeOutput(msg: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true) // append the content
        .create(true) // create if file doesn't exist
        .open("./output.txt")?;

    // a "closure"
    let time = {
        let local: DateTime<Local> = Local::now();
        format!("{}", local.format("%Y-%m-%d %H:%M:%S"))
    };
    let time = time.as_bytes();

    file.write_all("Time: ".as_bytes())?;
    file.write_all(time)?;
    file.write_all("\n".as_bytes())?;
    file.write_all(msg.as_bytes())?;

    file.flush()?;

    Ok(())
}
