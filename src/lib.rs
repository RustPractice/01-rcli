mod opts;
mod process;

pub use opts::{Opts, SubCommand};
pub use process::{process_csv, process_genpass};
// pub use opts::{Opts, SubCommand, OutputFormat};
// pub use process::process_csv;