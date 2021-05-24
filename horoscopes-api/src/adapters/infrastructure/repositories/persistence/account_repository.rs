use crate::db::MysqlPool;
use crate::domain::entities::account;
use crate::domain::repositories::AccountRepository;
use crate::models::{accounts::Accounts, users::Users};
use crate::schema::{
    accounts as accounts_schema, users as users_schema,
};

use diesel::prelude::*;
use std::sync::Arc;

pub struct AccountRepositoryImpl {
    pool: Arc<MysqlPool>,
}

impl AccountRepositoryImpl {
    pub fn new(pool: Arc<MysqlPool>) -> Self {
        Self { pool }
    }
}

impl AccountRepository for AccountRepositoryImpl {
    fn find_by_email(
        &self,
        email: account::Email,
    ) -> Option<account::Account> {
        let conn = self.pool.get().ok()?;

        let res_account = accounts_schema::dsl::accounts
            .filter(accounts_schema::email.eq(email.value()))
            .select((
                accounts_schema::id,
                accounts_schema::email,
                accounts_schema::password_hash,
                accounts_schema::user_id,
            ))
            .first::<Accounts>(&conn);

        if let Err(_) = res_account {
            return None;
        }

        let account = res_account.unwrap();

        let res_user = users_schema::dsl::users
            .filter(users_schema::id.eq(account.user_id))
            .select((users_schema::id,))
            .first::<Users>(&conn);

        if let Err(_) = res_user {
            return None;
        }

        let user = res_user.unwrap();

        // NOTE: unwarpでよいのか問題。
        let entity = account::Account::new(
            account.id,
            account.email,
            account.password_hash,
            user.id,
        )
        .unwrap();

        Some(entity)
    }

    fn store(&self, account: account::Account) -> Result<(), String> {
        let conn = self.pool.get().map_err(|err| {
            "Failed to get database connection from pool.".to_string()
        })?;

        let user_model = Users {
            id: account.user().id().value(),
        };

        let res_user = diesel::insert_into(users_schema::dsl::users)
            .values(user_model)
            .execute(&conn);

        if let Err(err) = res_user {
            return Err(err.to_string());
        }

        let account_model = Accounts {
            id: account.id().value(),
            email: account.email().value(),
            password_hash: account.password_hash().value(),
            user_id: account.user().id().value(),
        };

        let res_account =
            diesel::insert_into(accounts_schema::dsl::accounts)
                .values(account_model)
                .execute(&conn);

        match res_account {
            Ok(_res) => Ok(()),
            Err(err) => {
                println!("Err: {:?}", err);
                Err("Failed to store account.".to_string())
            }
        }
    }
}
