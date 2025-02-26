use rand::Rng;

use super::config::Config;

pub fn random_bytes(size: usize) -> Vec<u8> {
  let mut bytes = vec![0; size];
  rand::rng().fill(bytes.as_mut_slice());
  bytes
}

pub fn encrypt_password(config: &Config, input: &str) -> argon2::Result<String> {
  argon2::hash_encoded(
    input.as_bytes(),
    random_bytes(config.password.salt_length.into()).as_slice(),
    &argon2_config(config),
  )
}

pub fn verify_password(input: &str, encrypted_password: &str) -> argon2::Result<bool> {
  argon2::verify_encoded(encrypted_password, input.as_bytes())
}

fn argon2_config<'a>(config: &Config) -> argon2::Config<'a> {
  return argon2::Config {
    variant: argon2::Variant::Argon2id,
    hash_length: config.password.hash_length,
    lanes: config.password.parallelism,
    mem_cost: config.password.memory_mib * 1024,
    time_cost: config.password.iterations,
    ..Default::default()
  };
}
