extern crate serde;
extern crate serde_json;

pub use serde::Deserialize;

use std::{
    fs::File,
    io::BufReader,
    error::Error,
};

use super::planets::PlanetarySystem;

pub fn parse_init(fname: &str) -> Result<PlanetarySystem, Box<dyn Error>> {
    Ok(serde_json::from_reader(BufReader::new(File::open(fname)?))?)
}