use std::error::Error;
use std::fs::File;
use std::io::prelude::{Read, Write};

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path);
    let mut file = match file {
        Ok(v) => v,
        Err(e) => {
            let e = format!("failed to open file at {}: {}", path, e.to_string());
            return Err(e.into());
        }
    };

    let mut buf = Vec::new();
    if let Err(e) = file.read_to_end(&mut buf) {
        let e = format!("failed to read file at {}: {}", path, e.to_string());
        return Err(e.into());
    }

    Ok(buf)
}

pub fn write_to_file(path: &str, buf: &[u8]) -> Result<(), Box<dyn Error>> {
    let file = File::create(path);
    let mut file = match file {
        Ok(v) => v,
        Err(e) => {
            let e = format!("failed to create file at {}: {}", path, e.to_string());
            return Err(e.into());
        }
    };

    file.write_all(buf)?;

    Ok(())
}
