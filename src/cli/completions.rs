use std::io;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::core::error::InternalError;

use super::args::CliCommand;

pub async fn run(shell: Shell) -> Result<(), InternalError> {
  generate(
    shell,
    &mut CliCommand::command(),
    env!("CARGO_PKG_NAME"),
    &mut io::stdout(),
  );
  Ok(())
}
