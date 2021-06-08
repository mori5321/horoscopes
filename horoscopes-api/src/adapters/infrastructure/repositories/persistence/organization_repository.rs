use crate::db::DbPool;
use crate::domain::entities::organization::Organization;
use crate::domain::repositories::OrganizationRepository;
use crate::models::{
    organizations::Organizations,
    users_organizations::UsersOrganizations,
};
use crate::schema::{
    organizations as organizations_schema,
    users_organizations as users_organizations_schema,
};

use diesel::prelude::*;
use std::sync::Arc;

pub struct OrganizationRepositoryImpl {
    pool: Arc<DbPool>,
}

impl OrganizationRepositoryImpl {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl OrganizationRepository for OrganizationRepositoryImpl {
    fn store(
        &self,
        organization: &Organization,
    ) -> Result<(), String> {
        let conn = self.pool.get().unwrap();

        let organization_model = Organizations {
            id: organization.id().value(),
            name: organization.name().value(),
        };

        let users_organizations_models: Vec<UsersOrganizations> =
            organization
                .user_ids()
                .into_iter()
                .map(|user_id| UsersOrganizations {
                    user_id: user_id.value(),
                    organization_id: organization.id().value(),
                })
                .collect();

        let res =
            conn.transaction::<_, diesel::result::Error, _>(|| {
                diesel::insert_into(
                    organizations_schema::dsl::organizations,
                )
                .values(organization_model)
                .execute(&conn)?;

                diesel::insert_into(
                users_organizations_schema::dsl::users_organizations,
            )
            .values(users_organizations_models)
            .execute(&conn)?;

                return Ok(());
            });

        if let Err(err) = res {
            // TODO: SystemErrorを返却する。Usecase層でロギングする。
            println!("Err: {:?}", err);
            return Err(err.to_string());
        }

        Ok(())
    }
}
