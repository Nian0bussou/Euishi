use chrono::prelude::*;
use core::panic;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::Command;

pub fn make_output(msg: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./output_euishi.txt")?;

    write_file(
        &mut file,
        format!(
            "User : {}\nTime : {}\n{} ",
            g_user(),
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            msg
        ),
    )?;
    Ok(())
}

fn write_file<T: AsRef<str>>(f: &mut File, msg: T) -> std::io::Result<()> {
    f.write_all(msg.as_ref().as_bytes())?;
    f.flush()?;
    Ok(())
}

fn g_user() -> String {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command");

    let s = output.stdout;
    match String::from_utf8(s) {
        Ok(s) => s,
        _ => panic!(),
    }
}

#[test]
fn test_file() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./testFile.txt")?;
    write_file(&mut file, "This is a test\n")?;
    std::fs::remove_file("./testFile.txt")?;
    Ok(())
}
