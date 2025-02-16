use auth::{
  cli::{args::CliArgs, run::run},
  core::error::InternalError,
};

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), InternalError> {
  dotenvy::dotenv().ok();
  sqlx::any::install_default_drivers();

  run(CliArgs::parse()).await?;

  Ok(())
}
