extern crate serde_json;

use self::super::{Gitlab, Error, ErrorKind, Result};

/// Record representing 
pub struct GitIgnores<'a> {
    gitlab: &'a Gitlab,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    pub name: String,
    pub content: Option<String>,
}

impl<'a> GitIgnores<'a> {
    #[doc(hidden)]
    pub fn new(gitlab: &'a Gitlab, ) -> GitIgnores<'a> {
        GitIgnores {
            gitlab: gitlab,
        }
    }

    fn resource(&self, more: &str) -> String {
        format!("/templates/gitignores{}", more)
    }

    pub fn templates() {

    }

    pub fn single_template<T>(&self, name: T) -> Result<Template>
        where T: Into<String> {
        self.gitlab.get::<Template>(
            &self.resource(&format!("/{}", name.into()))
        )
    }
}