use std::fs::File;
use std::io::Write;

pub fn makeOutput(msg: String) -> std::io::Result<()> {
    let mut file = File::create("./output.txt")?;

    file.write_all(msg.as_bytes())?;

    file.flush()?;

    Ok(())
}
