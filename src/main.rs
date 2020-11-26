use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    verbose: bool,



    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Opt::from_args();
}
