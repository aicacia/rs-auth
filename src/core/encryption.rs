use rand::Rng;

use super::config::get_config;

pub fn generate_salt(salt: &mut [u8]) -> &[u8] {
  rand::thread_rng().fill(salt);
  salt
}

pub fn encrypt_password(input: &str) -> argon2::Result<String> {
  let config = get_config();
  argon2::hash_encoded(
    input.as_bytes(),
    generate_salt(&mut vec![0; config.password.salt_length.into()]),
    &argon2_config(),
  )
}

pub fn verify_password(input: &str, encrypted_password: &str) -> argon2::Result<bool> {
  argon2::verify_encoded(encrypted_password, input.as_bytes())
}

fn argon2_config<'a>() -> argon2::Config<'a> {
  let config = get_config();
  return argon2::Config {
    variant: argon2::Variant::Argon2id,
    hash_length: config.password.hash_length.into(),
    lanes: config.password.parallelism.into(),
    mem_cost: Into::<u32>::into(config.password.memory_mib) * 1024,
    time_cost: config.password.iterations.into(),
    ..Default::default()
  };
}
