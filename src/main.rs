mod app;
mod error;

pub(crate) use error::Result;

fn main() -> Result<()> {
    use structopt::StructOpt;
    let args = app::Options::from_args();

    match args.cmd {
        app::Command::Graph { adjacency_array } => {
            println!("Graph: {:?}", adjacency_array);
        },
    }
    Ok(())
}