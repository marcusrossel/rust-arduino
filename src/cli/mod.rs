//! This module provides an interface for interacting with the Arduino CLI.

mod run;
pub use run::*;

mod query;
pub use query::*;

/// The kinds of errors that can occur as a result of interacting with the Arduino CLI.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Error {
    CommandFailure,
    NoDevice,
    MultipleDevices,
    UnexpectedSyntax,
    InvalidSketchPath,
 }
