#[macro_use]
extern crate log;
extern crate failure;

use std::iter::FromIterator;
pub fn get_argv1() -> Result<String, failure::Error> {
    let argv = Vec::from_iter(std::env::args());
    debug!("Full argv is {:?}", argv);
    if argv.len() <= 1 {
        Err(failure::err_msg("Expected one argument"))
    } else {
        let arg = &argv[1];
        Ok(arg.to_string())
    }
}
