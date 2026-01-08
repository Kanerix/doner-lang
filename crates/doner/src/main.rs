//! The CLI used to run Döner lang.

use clap::{Parser, crate_authors, crate_version};
use clap_cargo::style::CLAP_STYLING;
use tracing_subscriber::EnvFilter;

mod error;
mod eval;

#[derive(Parser)]
#[command(
    name = "doner",
    about = "A command-line interface for döner lang",
    author = crate_authors!(),
    version = crate_version!(),
    after_help = "For more information, visit https://github.com/Kanerix/doner-lang",
    disable_version_flag = true,
    disable_help_subcommand = true,
    styles = CLAP_STYLING,
)]
struct Cli {
    /// Subcommands for the CLI
    #[command(subcommand)]
    command: Commands,

    /// Global arguments for all commands
    #[command(flatten)]
    global_args: Box<GlobalArgs>,
}

#[derive(clap::Subcommand)]
#[command(next_help_heading = "Commands", next_display_order = 2)]
enum Commands {
    /// Run a döner lang file
    Run(eval::RunArgs),
    /// Show the version of the CLI
    Version,
}

#[derive(clap::Parser, Debug)]
#[command(next_help_heading = "Global args", next_display_order = 1000)]
struct GlobalArgs {
    /// Enable verbose output.
    ///
    /// This will enable more detailed logging output, which can be useful for
    /// debugging. The maximum verbosity is level 3 (TRACE).
    ///
    /// Levels:
    /// - 0: No verbose output
    /// - 1: Info level output
    /// - 2: Debug level output
    /// - 3: Trace level output
    ///
    /// This uses the `RUST_LOG` environment variable to set the logging level.
    /// That means if the `RUST_LOG` environment variable is set, it will
    /// override the verbosity level set by this argument.
    #[arg(
        global = true,
        help = "Enable verbose output. Use multiple times for more verbosity.",
        short = 'v',
        env = "DONER_VERBOSE",
        action = clap::ArgAction::Count,
        verbatim_doc_comment,
    )]
    verbose: u8,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            match cli.global_args.verbose {
                0 => EnvFilter::from("none"),
                1 => EnvFilter::from("none,doner=info"),
                2 => EnvFilter::from("none,doner=debug"),
                _ => EnvFilter::from("none,doner=trace"),
            }
        }))
        .init();

    if std::env::var("RUST_LOG").is_ok() && cli.global_args.verbose > 0 {
        tracing::warn!(
            "RUST_LOG environment variable is present; this will override the \
            verbosity level set by the -v flag."
        );
    }

    let result = match cli.command {
        Commands::Run(run_args) => eval::eval(run_args, cli.global_args).await,
        Commands::Version => todo!(),
    };

    if let Err(err) = result {
        return Err(err.inner.into());
    };

    Ok(())
}
