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

fn parse_graph(adjacency_array: &str) -> Result<String, error::Error> {
    Ok(adjacency_array.to_string())
}

#[derive(StructOpt, Debug)]
enum Command {
    Graph {
        /// Graphs are represented as the upper half of an adjacency matrix collapsed to a single
        /// array
        ///
        /// This program expects undirected graphs represented as a single dimensional array. Start
        /// with the adjacency matrix of the graph. Since the graph is unweighted, we expect edges
        /// to be represented as "1" and an absence of an edge to be represented as "0". Since the
        /// graph is undirected we only need the upper triangular matrix to represent the whole
        /// graph. Then we encode it to a single dimension by the following algorithm:
        /// Add the top, left value to the array, then traverse the column from top to bottom.
        /// After the last value in the column, move back to the top and one position to the right.
        /// Continue adding values in this way until you've added the entire matrix to the array.
        #[structopt(name = "adjacency array", parse(try_from_str = parse_graph))]
        adjacency_array: String
    },
}

fn main() -> Result<()> {
    let args = Opt::from_args();
    match args.cmd {
        Command::Graph { adjacency_array } => {
            println!("Graph: {:?}", adjacency_array);
        },
    }
    Ok(())
}