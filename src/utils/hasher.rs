use actix::{Actor, SyncContext};
use libreauth::pass::{Algorithm, ErrorCode, HashBuilder, Hasher};

const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
const PWD_SCHEME_VERSION: usize = 1;

// This should never fail, unless it's given invalid parameters
pub fn hasher() -> Result<Hasher, ErrorCode> {
    // hasher configuration specifics
    HashBuilder::new()
        .algorithm(PWD_ALGORITHM)
        .version(PWD_SCHEME_VERSION)
        .finalize()
}
