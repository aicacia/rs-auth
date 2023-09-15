use rand::Rng;

pub fn generate_salt(salt: &mut [u8]) -> &[u8] {
  rand::thread_rng().fill(salt);
  salt
}

pub fn encrypt_password(input: &str) -> argon2::Result<String> {
  argon2::hash_encoded(
    input.as_bytes(),
    generate_salt(&mut [0u8; 32]),
    &argon2_config(),
  )
}

pub fn verify_password(input: &str, encrypted_password: &str) -> argon2::Result<bool> {
  argon2::verify_encoded(encrypted_password, input.as_bytes())
}

fn argon2_config<'a>() -> argon2::Config<'a> {
  return argon2::Config {
    variant: argon2::Variant::Argon2id,
    hash_length: 32,
    lanes: 8,
    mem_cost: 16 * 1024,
    time_cost: 8,
    ..Default::default()
  };
}
