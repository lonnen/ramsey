mod error;

use std::path::PathBuf;
use structopt::StructOpt;

pub(crate) use error::{Error, Result};

#[derive(StructOpt, Debug)]
struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    verbose: bool,

    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let options = Opt::from_args();
    println!("{:?}", options);
    Ok(())
}