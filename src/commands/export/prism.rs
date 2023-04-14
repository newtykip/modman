use crate::Error;
use std::{fs::File, io::Write};
use zip::{write::FileOptions, ZipWriter};

pub fn execute() -> Result<(), Error> {
    let dir = std::env::current_dir()?;
    let name = "hi";

    let mut file = File::create(dir.join(format!("{}.zip", name)))?;
    let mut zip = ZipWriter::new(&mut file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    // instance.cfg
    zip.start_file("instance.cfg", options)?;
    zip.write(
        format!(
            "name={}
iconKey=default",
            name
        )
        .as_bytes(),
    )?;

    zip.finish()?;

    Ok(())
}
