use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::articles::comments::{CreateCommentOuter, DeleteComment, GetComments};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{Comment, NewComment};
use crate::prelude::*;
use crate::utils::CustomDateTime;

// message handler implementations ↓
