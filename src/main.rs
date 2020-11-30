mod error;

use structopt::StructOpt;

pub(crate) use error::Result;

#[derive(StructOpt, Debug)]
struct Opt {

    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long)]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Graph {
        adjacency: String
    },
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    match args.cmd {
        _ => {
            Opt::clap().print_help().unwrap()
        }
    }
    Ok(())
}