use crate::domain::entities::user::User;
use crate::domain::repositories::UserRepository;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct UserRepositoryOnMemory {}

pub static USERS_ON_MEMORY: Lazy<Mutex<Vec<User>>> =
    Lazy::new(|| {
        let users = vec![User::new("user-0001".to_string())];

        Mutex::new(users)
    });

// impl UserRepositoryOnMemory {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

impl UserRepository for UserRepositoryOnMemory {
    fn store(&self, user: User) -> Result<(), String> {
        let mut users = USERS_ON_MEMORY.lock().unwrap();

        let opt_idx =
            users.clone().into_iter().position(|u| u == user);

        match opt_idx {
            Some(idx) => {
                users
                    .splice(idx..idx + 1, vec![user].iter().cloned());
                println!("Users: {:?}", users);
                Ok(())
            }
            None => {
                users.push(user.clone());
                println!("Users: {:?}", users);
                Ok(())
            }
        }
    }
}
