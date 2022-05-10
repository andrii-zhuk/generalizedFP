use std::{io::{Result, Write, BufWriter}, fs::File, env::consts::OS};

use crate::types::DirectedGraph;

pub fn to_file(path: &String, graph: &DirectedGraph) -> Result<()> {
    let mut file = File::open(path);
    if file.is_err() {
        let err = file.unwrap_err();
        if err.kind() == std::io::ErrorKind::NotFound {
            file = File::create(path);
        }
        else {
            return Err(err)
        }
    }
    let file = file?;

    let buf_writer = BufWriter::new(file);

    serde_json::to_writer(buf_writer, graph)?;


    Ok(())
} 