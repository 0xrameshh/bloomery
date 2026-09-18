use clap::Parser;
use tracing_subscriber::EnvFilter;

use bloomery::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Extract {
            input,
            out,
            model,
            no_clean,
            keep_incomplete,
            concurrency,
            report,
            quiet,
            staging_dir,
            ..
        } => bloomery::commands::handle_extract(
            input,
            out,
            model,
            no_clean,
            keep_incomplete,
            concurrency,
            report,
            quiet,
            staging_dir,
        ),
        Commands::Convert {
            input,
            out,
            model,
            concurrency,
            report,
            quiet,
        } => bloomery::commands::handle_convert(input, out, model, concurrency, report, quiet),
        Commands::Clean { input, out } => bloomery::commands::handle_clean(input, out),
        Commands::Info { file } => bloomery::commands::handle_info(file),
        Commands::Find { all } => bloomery::commands::handle_find(all),
        Commands::Verify {
            input,
            golden,
            provider: _provider,
        } => bloomery::commands::handle_verify(input, golden),
        Commands::Generate { out, count, .. } => bloomery::commands::handle_generate(&out, count),
        Commands::Studio { input, port } => bloomery::commands::handle_studio(&input, port),
    }
}
