use crate::domain::entities::todo::{Todo, ID};

pub trait TodoRepository: Send + Sync {
    fn list(&self) -> Vec<Todo>;
    fn find(&self, id: ID) -> Option<Todo>;
    fn store(&self, todo: Todo) -> Result<(), String>; // ErrorはStringで仮置き
    fn remove(&self, id: ID) -> Result<(), String>; // ErrorはStringで仮置き
}
