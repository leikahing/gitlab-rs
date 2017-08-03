//! Result wrappers and Gitlab errors.

use std::io::Error as IoError;
use reqwest::Error as HttpError;
use reqwest::StatusCode;
use serde_json::error::Error as SerdeError;

#[derive(Debug, Deserialize, PartialEq)]
pub struct UnknownRoute {
    pub error: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GitlabError {
    pub message: String,
}

error_chain! {
    errors {
        Fault {
            code: StatusCode,
            error: String,
        }
    }
    foreign_links {
        Codec(SerdeError);
        Http(HttpError);
        IO(IoError);
    }
}