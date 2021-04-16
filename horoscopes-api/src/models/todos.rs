#[derive(Queryable, Clone)]
pub struct Todos {
    pub id: String,
    pub title: String,
    pub done: bool,
}
