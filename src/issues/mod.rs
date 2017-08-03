extern crate serde_json;

use self::super::{Gitlab, Error, ErrorKind, Result};

/// Struct representing a client for Issues
pub struct Issues<'a> {
    gitlab: &'a Gitlab,
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    
}