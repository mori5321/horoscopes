#[derive(Clone, Debug)]
pub struct User {
    id: ID,
    name: Name,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id: ID::new(id),
            name: Name::new(name),
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    fn new(name: String) -> Self {
        Name(name)
    }
}
