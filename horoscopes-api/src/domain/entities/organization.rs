use super::user::ID as UserID;

#[derive(Clone, Debug)]
pub struct Organization {
    id: ID,
    name: Name,
    user_ids: Vec<UserID>,
}

impl Organization {
    pub fn new(
        id: String,
        name: String,
        user_ids: Vec<String>,
    ) -> Self {
        Self {
            id: ID::new(id),
            name: Name::new(name),
            user_ids: user_ids
                .into_iter()
                .map(|id| UserID::new(id))
                .collect(),
        }
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn user_ids(&self) -> Vec<UserID> {
        self.user_ids.clone()
    }

    pub fn add_user(&mut self, user_id: UserID) {
        self.user_ids.push(user_id)
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
    pub fn new(name: String) -> Self {
        Name(name)
    }

    pub fn value(&self) -> String {
        let Name(name) = self;
        name.clone()
    }
}
