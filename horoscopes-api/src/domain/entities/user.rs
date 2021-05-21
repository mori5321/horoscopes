#[derive(Clone, Debug)]
pub struct User {
    id: ID,
}

impl User {
    pub fn new(id: String) -> Self {
        Self { id: ID::new(id) }
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ID(String);

impl ID {
    pub fn new(id: String) -> Self {
        ID(id)
    }

    pub fn value(&self) -> String {
        let ID(id) = self;
        id.clone()
    }
}
