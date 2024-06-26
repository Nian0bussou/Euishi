use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn makeOutput(msg: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./output_euishi.txt")?;

    file_writer(
        &mut file,
        format!(
            "Time : {}\n{}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            msg
        ),
    )?;
    Ok(())
}

fn file_writer<T>(f: &mut File, msg: T) -> std::io::Result<()>
where
    T: AsRef<str>,
{
    f.write_all(msg.as_ref().as_bytes())?;
    f.flush()?;
    Ok(())
}

//

#[test]
fn testFile() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./testFile.txt")?;

    file_writer(&mut file, "This is a test\n")?;

    Ok(())
}
