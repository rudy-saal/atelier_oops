use std::sync::Arc;

use clap::Parser;
use cli::Args;
use client::HttpPingClient;
use exec::Executor;
use logging::ConsoleStatsLogger;
use response::ResponseHandler;

mod cli;
mod client;
mod exec;
mod logging;
mod response;
mod stats;

fn main() {
    let client = Arc::from(HttpPingClient {});
    let handler = Arc::from(ResponseHandler {});
    let logger = Arc::from(ConsoleStatsLogger {});
    let executor = Executor::new(client, handler, logger);

    let args = Args::parse();
    executor.execute(&args.url, args.tries);
}
