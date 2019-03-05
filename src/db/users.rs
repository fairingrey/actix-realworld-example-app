use crate::db::{may_update, Conn};
use crate::models::{CredentialChange, NewCredential, NewUser, User, UserChange};
use crate::prelude::*;
use crate::schema::{credentials, users};
use crate::utils::hasher;
use diesel::prelude::*;
