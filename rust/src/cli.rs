use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// URL to ping
    #[arg(short, long)]
    pub url: String,

    /// Number of tries
    #[arg(short, long, default_value_t = 20)]
    pub tries: u32,
}
