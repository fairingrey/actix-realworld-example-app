use libreauth::pass::{Algorithm, HashBuilder, Hasher};

pub const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;
pub const PWD_SCHEME_VERSION: usize = 1;

// If the Hasher changes, make sure to increment PWD_SCHEME_VERSION
lazy_static! {
    pub static ref HASHER: Hasher = {
        HashBuilder::new()
            .algorithm(PWD_ALGORITHM)
            .version(PWD_SCHEME_VERSION)
            .finalize()
            .unwrap()
    };
}
