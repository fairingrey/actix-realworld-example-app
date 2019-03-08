use libreauth::pass::{Algorithm, HashBuilder, Hasher};

pub const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
pub const PWD_SCHEME_VERSION: usize = 1;

// This should never fail, unless it's given invalid parameters
// If the HashBuilder changes, make sure to increment PWD_SCHEME_VERSION
pub fn hasher() -> Hasher {
    // hasher configuration specifics
    HashBuilder::new()
        .algorithm(PWD_ALGORITHM)
        .version(PWD_SCHEME_VERSION)
        .finalize()
        .unwrap()
}
