use std::{
    env::consts::OS,
    fs::File,
    io::{BufWriter, Result, Write},
};

use crate::types::{algorithm_result::AlgorithmResult, DirectedGraph};

fn open_or_create_file(path: &String) -> Result<BufWriter<File>> {
    let file = File::create(path)?;

    return Ok(BufWriter::new(file));
}
pub fn graph_to_file(path: &String, graph: &DirectedGraph) -> Result<()> {
    let buf_writer = open_or_create_file(path)?;

    serde_json::to_writer(buf_writer, graph)?;

    Ok(())
}

pub fn algorithm_steps_to_file(path: &String, algorithm_result: &AlgorithmResult) -> Result<()> {
    let buf_writer = open_or_create_file(path)?;

    serde_json::to_writer(buf_writer, algorithm_result)?;

    Ok(())
}
