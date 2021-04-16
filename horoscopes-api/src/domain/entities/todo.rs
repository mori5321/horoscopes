use std::cmp::Eq;

#[derive(Clone)]
pub struct Todo {
    pub id: ID,
    pub title: Title,
    pub is_done: IsDone,
}

// 全体的にValueObjectの知識がEntityの外に出ていくのはよくないかもしれない...?
// - ValueObjectの生成・操作・アクセスはEntityからのみにすべき?
// - そうするとリポジトリの引数などは、Primitiveタイプなどを使うべきなのか...?
//
// 一旦はPublicにする方針で良しとする
// getterを逐一用意する & repositoryなどではPrimitiveタイプを使う、という方針は成り立ちはしそう
// 良いかはわからない。

impl Todo {
    pub fn new(id: String, title: String, is_done: bool) -> Self {
        Self {
            id: ID::new(id),
            title: Title::new(title),
            is_done: IsDone::new(is_done),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ID(String);

// ValueObjectの実装参考
// https://github.com/nrslib/BottomUpDDDTheLaterPart/blob/master/src/Domain/Domain/Users/UserId.cs
impl ID {
    fn new(id: String) -> Self {
        ID(id)
    }

    pub fn to_string(&self) -> String {
        let ID(id) = self;
        id.clone()
    }

    pub fn from_string(string: String) -> ID {
        // TODO: Validation
        ID(string)
    }
}

#[derive(Clone)]
pub struct Title(String);

impl Title {
    fn new(text: String) -> Self {
        Title(text)
    }

    pub fn to_string(&self) -> String {
        let Title(text) = self;
        text.clone()
    }
}

#[derive(Clone)]
pub struct IsDone(bool);

impl IsDone {
    fn new(is_done: bool) -> Self {
        IsDone(is_done)
    }

    pub fn to_bool(&self) -> bool {
        let IsDone(is_done) = self;
        is_done.clone()
    }
}
