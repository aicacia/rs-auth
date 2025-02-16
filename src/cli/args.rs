use clap::Parser;
#[cfg(feature = "completions")]
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct CliArgs {
  #[arg(long, short = 'c', default_value = "./config.json")]
  pub config: String,
  #[clap(subcommand)]
  pub command: Option<CliCommand>,
}

#[derive(Parser, Debug)]
pub enum CliCommand {
  Serve {
    #[clap(flatten)]
    serve: CliServe,
  },
  #[cfg(feature = "completions")]
  Completions { shell: Shell },
}

#[derive(Parser, Debug, Default)]
pub struct CliServe {}
