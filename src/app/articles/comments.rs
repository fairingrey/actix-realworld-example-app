use actix_web::{HttpRequest, HttpResponse, Json, Path, ResponseError};
use futures::{future::result, Future};
use validator::Validate;

use super::super::AppState;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

// Extractors ↓

// Client Messages ↓

// JSON response objects ↓

// Route handlers ↓
