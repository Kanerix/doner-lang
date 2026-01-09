pub mod error;

pub use crate::error::Result;

use doner_parser::Program;

pub fn eval(_ast: Program) -> Result<()> {
    todo!()
}
