use std::path::PathBuf;

use clap::Clap;

use hyperqueue::client::commands::stats::get_server_stats;
use hyperqueue::client::commands::stop::stop_server;
use hyperqueue::client::commands::submit::submit_computation;
use hyperqueue::common::setup::setup_logging;
use hyperqueue::server::bootstrap::{init_hq_server, get_client_connection};
use hyperqueue::common::fsutils::absolute_path;
use hyperqueue::worker::start::{start_hq_worker, WorkerStartOpts};
use hyperqueue::WorkerId;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub type Connection = tokio_util::codec::Framed<tokio::net::UnixStream, tokio_util::codec::LengthDelimitedCodec>;

#[derive(Clap)]
struct CommonOpts {
    #[clap(long)]
    server_dir: Option<PathBuf>,
}


#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(setting = clap::AppSettings::ColoredHelp)]
struct Opts {
    #[clap(flatten)]
    common: CommonOpts,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl CommonOpts {
    fn get_server_directory_path(&self) -> PathBuf {
        absolute_path(self.server_dir.clone().unwrap_or_else(default_server_directory_path))
    }
}

#[derive(Clap)]
struct ServerStartOpts {
}

#[derive(Clap)]
struct ServerStopOpts {
}

#[derive(Clap)]
struct StatsOpts {
}

#[derive(Clap)]
struct SubmitOpts {
    commands: Vec<String>,
}

#[derive(Clap)]
enum SubCommand {
    Server(ServerOpts),
    Stats(StatsOpts),
    Submit(SubmitOpts),
    Worker(WorkerOpts),
}

#[derive(Clap)]
struct ServerOpts {
    #[clap(subcommand)]
    subcmd: ServerCommand,
}

#[derive(Clap)]
enum ServerCommand {
    Start(ServerStartOpts),
    Stop(ServerStopOpts),
}

#[derive(Clap)]
struct WorkersOpts {

}

#[derive(Clap)]
struct WorkerOpts {
    #[clap(subcommand)]
    subcmd: WorkerCommand,
}

#[derive(Clap)]
struct WorkerStopOpts {
    worker_id: WorkerId,
}

#[derive(Clap)]
struct WorkerListOpts {
}

#[derive(Clap)]
struct WorkerInfoOpts {
    worker_id: WorkerId,
}


#[derive(Clap)]
enum WorkerCommand {
    Start(WorkerStartOpts),
    Stop(WorkerStopOpts),
    List(WorkerListOpts),
    Info(WorkerInfoOpts),
}

async fn command_server_start(common: CommonOpts, opts: ServerStartOpts) -> hyperqueue::Result<()> {
    init_hq_server(&common.get_server_directory_path()).await
}

async fn command_server_stop(common: CommonOpts, opts: ServerStopOpts) -> hyperqueue::Result<()> {
    let mut connection = get_client_connection(&common.get_server_directory_path()).await?;
    stop_server(&mut connection).await
}

async fn command_stats(common: CommonOpts, opts: StatsOpts) -> hyperqueue::Result<()> {
    let mut connection = get_client_connection(&common.get_server_directory_path()).await?;
    get_server_stats(&mut connection).await
}

async fn command_submit(common: CommonOpts, opts: SubmitOpts) -> hyperqueue::Result<()> {
    let mut connection = get_client_connection(&common.get_server_directory_path()).await?;
    submit_computation(&mut connection, opts.commands).await
}

async fn command_worker(common: CommonOpts, opts: WorkerStartOpts) -> hyperqueue::Result<()> {
    start_hq_worker(&common.get_server_directory_path(), opts).await
}

fn default_server_directory_path() -> PathBuf {
    let mut home = dirs::home_dir().unwrap_or_else(std::env::temp_dir);
    home.push(".hq-server");
    home
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> hyperqueue::Result<()> {
    let top_opts: Opts = Opts::parse();
    setup_logging();

    let result = match top_opts.subcmd {
        SubCommand::Server(ServerOpts { subcmd: ServerCommand::Start(opts) }) => command_server_start(top_opts.common, opts).await,
        SubCommand::Server(ServerOpts { subcmd: ServerCommand::Stop(opts) }) => command_server_stop(top_opts.common, opts).await,

        SubCommand::Worker(WorkerOpts { subcmd: WorkerCommand::Start(opts) }) => { command_worker(top_opts.common, opts).await },
        SubCommand::Worker(WorkerOpts { subcmd: WorkerCommand::Stop(_) }) => { todo!() }
        SubCommand::Worker(WorkerOpts { subcmd: WorkerCommand::List(_) }) => { todo!() }
        SubCommand::Worker(WorkerOpts { subcmd: WorkerCommand::Info(_) }) => { todo!() }

        SubCommand::Stats(opts) => command_stats(top_opts.common, opts).await,
        SubCommand::Submit(opts) => command_submit(top_opts.common, opts).await,

    };
    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    Ok(())
}
