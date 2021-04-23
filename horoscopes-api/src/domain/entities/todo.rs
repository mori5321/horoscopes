use std::cmp::Eq;
use crate::domain::errors::{DomainError, DomainErrorType};

#[derive(Clone, Debug)]
pub struct Todo {
    id: ID,
    title: Title,
    is_done: IsDone,
}

// 全体的にValueObjectの知識がEntityの外に出ていくのはよくないかもしれない...?
// - ValueObjectの生成・操作・アクセスはEntityからのみにすべき?
// - そうするとリポジトリの引数などは、Primitiveタイプなどを使うべきなのか...?
//
// 一旦はPublicにする方針で良しとする
// getterを逐一用意する & repositoryなどではPrimitiveタイプを使う、という方針は成り立ちはしそう
// 良いかはわからない。

impl Todo {
    pub fn new(id: String, title: String, is_done: bool) -> Result<Self, DomainError> {
        let res_title = Title::new(title);
        
        if let Err(err) = res_title {
            return Err(err)
        }

        let todo = Self {
            id: ID::new(id),
            title: res_title.unwrap(),
            is_done: IsDone::new(is_done),
        };

        return Ok(todo)
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
    fn new(text: String) -> Result<Self, DomainError> {
        // 今回の場合、文字数バリデーションをここのレイヤーに書くのはまちがい。
        // なぜならば、「保存できる文字数をもっと減らしたい」となった場合に
        // 旧データがコンストラクトできなくなってしまう。
        if text.len() > TITLE_MAX_LENGTH {
            return Err(
                DomainError {
                    err_type: DomainErrorType::ExceedMaxLengthError,
                    message: format!("Title must be less than {} letters", TITLE_MAX_LENGTH)
                }
            )
        }

        Ok(Title(text))
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
