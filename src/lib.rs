//! gitlab is a library for interacting with the Gitlab v4 API.

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

extern crate reqwest;

#[macro_use] extern crate hyper;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate url;

pub mod errors;
pub mod projects;
pub mod gitignores;

//pub mod ci_lint;
pub use errors::{Error, ErrorKind, Result};

use serde::de::DeserializeOwned;

use gitignores::GitIgnores;
use projects::Projects;

use reqwest::Body;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Method;
use reqwest::header::{Headers, Authorization, Bearer, ContentType, Accept, qitem};
use reqwest::mime;

use std::io::Read;

header! { (GitlabPrivateToken, "PRIVATE-TOKEN") => [String] }

/// Credentials for Gitlab authentication
#[derive(Debug, PartialEq)]
pub enum Credentials {
    /// Unauthenticated anonymous access,
    Anonymous,

    /// OAuth2 token
    OauthToken(String),

    /// Personal Access / Private Token
    AccessToken(String)
}

impl Default for Credentials {
    fn default() -> Credentials {
        Credentials::Anonymous
    }
}

/// Gitlab API client
pub struct Gitlab {
    host: String,
    http: Client,
    credentials: Credentials,
}

impl Gitlab {
    /// Create a Gitlab client.
    pub fn new<T>(host: T, http: Client, credentials: Credentials) -> Gitlab
        where T: Into<String> {
        Gitlab {
            host: host.into() + "/api/v4",
            http: http,
            credentials: credentials,
        }
    }

    pub fn gitignores(&self) -> GitIgnores {
        GitIgnores::new(self)
    }

    pub fn projects(&self) -> Projects {
        Projects::new(self)
    }

    fn request_headers(&self) -> Headers {
        let mut h = Headers::with_capacity(5);
        h.set(ContentType(mime::APPLICATION_JSON));
        h.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
        match self.credentials {
            Credentials::OauthToken(ref token) => {
                h.set(Authorization(
                    Bearer {
                        token: token.clone()
                    }
                ))
            }
            Credentials::AccessToken(ref token) => {
                h.set(GitlabPrivateToken(token.clone()))
            }
            _ => { ; }
        };
        h
    }

    fn get<T>(&self, resource: &str) -> Result<T>
        where T: DeserializeOwned,
    {
        let url = format!("{}{}", self.host, resource);
        println!("URL: {}", url);

        let mut rsp = self.http.get(&*url)?
            .headers(self.request_headers())
            .send()?;

        /*let mut content = String::new();
        rsp.read_to_string(&mut content);
        println!("{}", content);*/
        match rsp.status() {
            StatusCode::BadRequest |
            StatusCode::Unauthorized |
            StatusCode::Forbidden |
            StatusCode::NotFound |
            StatusCode::MethodNotAllowed |
            StatusCode::Conflict |
            StatusCode::UnprocessableEntity |
            StatusCode::InternalServerError => {
                Err(
                    ErrorKind::Fault {
                        // TODO Finalize JSON parsing for Error messages
                        code: rsp.status(),
                        error: String::from("Problem")
                    }.into(),
                )
            }
            _ => {
                Ok(rsp.json()?)
            }
        }
    }

    fn post<T>(&self, resource: &str, body: Vec<u8>) -> Result<T>
        where T: DeserializeOwned,
    {
        let url = format!("{}{}", self.host, resource);
        println!("Post URL: {}", url);

        let mut rsp = self.http.post(&*url)?
            .headers(self.request_headers())
            .body(body)
            .send()?;

        /*let mut content = String::new();
        rsp.read_to_string(&mut content);
        println!("{}", content);*/
        match rsp.status() {
            StatusCode::BadRequest |
            StatusCode::Unauthorized |
            StatusCode::Forbidden |
            StatusCode::NotFound |
            StatusCode::MethodNotAllowed |
            StatusCode::Conflict |
            StatusCode::UnprocessableEntity |
            StatusCode::InternalServerError => {
                Err(
                    ErrorKind::Fault {
                        // TODO Finalize JSON parsing for Error messages
                        code: rsp.status(),
                        error: String::from("Problem")
                    }.into(),
                )
            }
            _ => {
                Ok(rsp.json()?)
            }
        }
    }

    fn delete(&self, resource: &str) {
        let url = format!("{}{}", self.host, resource);
        println!("Delete URL: {}", url);

        let mut rsp = self.http.delete(&*url).unwrap()
            .headers(self.request_headers())
            .send().unwrap();
        
        let mut content = String::new();
        rsp.read_to_string(&mut content);
        println!("{}", content);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
