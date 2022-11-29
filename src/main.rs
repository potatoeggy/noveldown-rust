use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(help = "Novel to download")]
    novel_id: String,
    #[arg(help = "The path to download to [default: current dir]")]
    path: Option<std::path::PathBuf>,

    #[arg(long, help = "Start chapter (inclusive)")]
    start: Option<u32>,

    #[arg(long, help = "End chapter (inclusive)")]
    end: Option<u32>,

    #[arg(short, long, help = "Print supported IDs")]
    supported_ids: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.start)
}
