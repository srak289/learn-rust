use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    display: Option<String>,
}
