#[derive(Clone, Debug)]
pub struct Account {
    id: ID,
    email: Email,
    password_hash: PasswordHash,
}

impl Account {
    pub fn new(
        id: String,
        email: String,
        password_hash: String,
    ) -> Self {
        Self {
            id: ID::new(id),
            email: Email::new(email),
            password_hash: PasswordHash::new(password_hash),
        }
    }

    pub fn from_signup(
        signup: &SignUp,
        password_hash: String,
    ) -> Self {
        Account {
            id: signup.id.clone(),
            email: signup.email.clone(),
            password_hash: PasswordHash(password_hash),
        }
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password_hash(&self) -> PasswordHash {
        self.password_hash.clone()
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// - a. Hash変換をドメインサービスで行ってAccountに変換する。その後repositoryでAccountをstoreする
//      - SignUpAccount, LoginAccountに分割するというのはありな気がする
// - b. repositoryでWriteAccountをstoreする。repositoryの実体内でhash変換する。
//      - これの問題点はpasswordのverify処理をrepositoryには書けないこと。
//      - 必然的にdomain serviceにveriry, repositoryにhashといった具合にpassword logicがちらばる。

#[derive(Clone, Debug)]
pub struct SignUp {
    id: ID,
    email: Email,
    password: Password,
    password_confirmation: Password,
}

#[derive(Clone, Debug)]
pub struct Login {
    email: Email,
    password: Password,
}

// Memo:
//
// AccountService::is_duplicated(signup)
// AccountService::to_account(signup)
// AccountService::verify(login, account)

impl SignUp {
    pub fn new(
        id: String,
        email: String,
        password: String,
        password_confirmation: String,
    ) -> Self {
        let account = Self {
            id: ID::new(id),
            email: Email::new(email),
            password: Password::new(password),
            password_confirmation: Password::new(
                password_confirmation,
            ),
        };

        return account;
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }

    pub fn password_confirmation(&self) -> Password {
        self.password_confirmation.clone()
    }
}

impl Login {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email: Email::new(email),
            password: Password::new(password),
        }
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn password(&self) -> Password {
        self.password.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ID(String);

impl ID {
    fn new(id: String) -> Self {
        ID(id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Email(String);

impl Email {
    fn new(email: String) -> Self {
        Email(email)
    }

    pub fn from_string(s: String) -> Self {
        Self::new(s)
    }

    pub fn value(&self) -> String {
        let Self(s) = self;
        return s.clone();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    fn new(password: String) -> Self {
        Password(password)
    }

    pub fn value(&self) -> String {
        let Password(password) = self;
        password.clone()
    }

    // ここに持たせるのは微妙
    // fn to_hash(&self, service: PasswordService) -> PasswordHash {
    //   service.to_hash(password);
    // }
    //
}

#[derive(Clone, Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password_hash: String) -> Self {
        PasswordHash(password_hash)
    }

    pub fn value(&self) -> String {
        let PasswordHash(hash) = self;
        hash.clone()
    }

    // ここに持たせるのは微妙
    // fn match(&self, password: Password, service: PasswordService) -> bool {
    //   let hash = self.value;
    //   service.verify(hash, password)
    // }
    //
    // あるいはPasswordServiceをUsecaseから呼び出してしまったほうが健康的な気もする。
}
