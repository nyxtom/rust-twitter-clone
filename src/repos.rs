use std::io::Error;

pub mod user;

pub trait Store<Id, T> {
    fn insert(&mut self, item: T) -> Result<T, Error>;
    fn get_by_id(&self, id: Id) -> Result<T, Error>;
    fn list(&self) -> Result<Vec<T>, Error>;
}

pub trait UniqueId<T> {
    fn get_id<'item>(&'item self) -> Option<&'item T>;
}

#[derive(Debug, Clone)]
pub struct MemoryStore<T> {
    pub cache: Vec<T>,
}

impl<T: Clone + UniqueId<String>> MemoryStore<T> {
    pub fn new() -> Self {
        MemoryStore { cache: Vec::new() }
    }
}

impl<T: Clone + UniqueId<String>> Store<String, T> for MemoryStore<T> {
    fn insert(&mut self, item: T) -> Result<T, Error> {
        self.cache.push(item.clone());
        Ok(item)
    }

    fn get_by_id(&self, id: String) -> Result<T, Error> {
        if let Some(result) = self.cache.iter().find(|p| p.get_id().eq(&Some(&id))) {
            Ok(result.clone())
        } else {
            Err(Error::new(std::io::ErrorKind::NotFound, "Not found"))
        }
    }

    fn list(&self) -> Result<Vec<T>, Error> {
        Ok(self.cache.clone())
    }
}
