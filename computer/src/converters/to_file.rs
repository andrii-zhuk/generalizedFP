use std::{
    fs::File,
    io::{BufWriter, Result},
};

use serde::Serialize;

fn open_or_create_file(path: &String) -> Result<BufWriter<File>> {
    let file = File::create(path)?;

    return Ok(BufWriter::new(file));
}

pub fn write_to_file<T>(path: &String, value: &T) -> Result<()>
where
    T: ?Sized + Serialize,
{
    let buf_writer = open_or_create_file(path)?;

    serde_json::to_writer(buf_writer, value)?;

    Ok(())
}
