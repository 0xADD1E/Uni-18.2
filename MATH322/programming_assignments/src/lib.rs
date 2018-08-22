extern crate failure;

use std::io::Read;
use std::iter::FromIterator;
pub fn get_argv1() -> Result<String, failure::Error> {
    let argv = Vec::from_iter(std::env::args());
    if argv.len() <= 1 {
        Err(failure::err_msg("Expected one argument"))
    } else {
        let arg = &argv[1];
        Ok(arg.to_string())
    }
}

pub fn ch09_relations(filename: &std::path::Path) -> Result<(), failure::Error> {
    let file = std::fs::File::open(filename)?;
    println!("{:?}", file);
    Ok(())
}
