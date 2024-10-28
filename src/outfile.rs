use chrono::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
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
    let output = Command::new("whoami").output();

    if let Ok(x) = output {
        let s = x.stdout;
        if let Ok(s) = String::from_utf8(s) {
            s
        } else {
            "----------".to_owned()
        }
    } else {
        "----------".to_owned()
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
