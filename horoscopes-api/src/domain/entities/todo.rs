use std::cmp::Eq;

#[derive(Clone, Debug)]
pub struct Todo {
    id: ID,
    title: Title,
    is_done: IsDone,
}

impl Todo {
    pub fn new(id: String, title: String, is_done: bool) -> Self {
        let todo = Self {
            id: ID::new(id),
            title: Title::new(title),
            is_done: IsDone::new(is_done),
        };

        return todo
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }

    pub fn title(&self) -> Title {
        self.title.clone()
    }

    pub fn is_done(&self) -> IsDone {
        self.is_done.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ID(String);

// ValueObjectの実装参考
// https://github.com/nrslib/BottomUpDDDTheLaterPart/blob/master/src/Domain/Domain/Users/UserId.cs
impl ID {
    fn new(id: String) -> Self {
        ID(id)
    }

    pub fn value(&self) -> String {
        self.to_string()
    }
    
    pub fn from_str(str: &str) -> ID {
        Self::new(str.to_string())
    }

    fn to_string(&self) -> String {
        let ID(id) = self;
        id.clone()
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Clone, Debug)]
pub struct Title(String);

const TITLE_MAX_LENGTH: usize = 80;

impl Title {
    fn new(text: String) -> Self {
        Title(text)
    }

    pub fn value(&self) -> String {
        self.to_string()
    }
    
    fn to_string(&self) -> String {
        let Title(text) = self;
        text.clone()
    }
}

#[derive(Clone, Debug)]
pub struct IsDone(bool);

impl IsDone {
    fn new(is_done: bool) -> Self {
        IsDone(is_done)
    }

    pub fn value(&self) -> bool {
        let IsDone(is_done) = self;
        is_done.clone()
    }
}
