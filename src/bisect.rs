use std::error::Error as StdError;
use crate::gnuplot::PlotData;

pub fn bisect(data: PlotData) -> Result<(), Error> {
    panic!()
}

#[derive(Display, Debug)]
pub enum Error {
}

impl StdError for Error { }
