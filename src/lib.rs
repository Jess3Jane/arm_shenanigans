extern crate tempdir;
extern crate unicorn;

mod preprocess;
pub use self::preprocess::Preprocessor;

mod assembler;
pub use self::assembler::{assemble, AssemblerError};

pub mod util;
