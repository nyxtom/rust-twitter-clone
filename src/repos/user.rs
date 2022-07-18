use std::io::Error;

use serde::{Deserialize, Serialize};

use super::{MemoryStore, Store, UniqueId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub totp_enabled: bool,
    #[serde(skip)]
    pub totp_secret: Option<String>,
}

impl UniqueId<String> for User {
    fn get_id<'user>(&'user self) -> Option<&'user String> {
        Some(&self._id)
    }
}

pub trait UserStore: Store<String, User> {
    fn authenticate(&self, username: String, password: String) -> Result<User, Error>;
}

impl UserStore for MemoryStore<User> {
    fn authenticate(&self, username: String, password: String) -> Result<User, Error> {
        if let Some(result) = self
            .cache
            .iter()
            .find(|p| p.username == username && p.password == password)
        {
            Ok(result.clone())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Not found"))
        }
    }
}
