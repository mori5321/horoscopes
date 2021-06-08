use crate::db::DbPool;
use crate::domain::entities::diagnosis::{self, Diagnosis};
use crate::domain::repositories::DiagnosisRepository;
use crate::models::diagnoses::{to_entity, Diagnoses};
use crate::schema::diagnoses as diagnoses_schema;

use diesel::prelude::*;
use std::sync::Arc;

pub struct DiagnosisRepositoryImpl {
    pool: Arc<DbPool>,
}

impl DiagnosisRepositoryImpl {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

impl DiagnosisRepository for DiagnosisRepositoryImpl {
    fn list(&self) -> Vec<Diagnosis> {
        let conn = self.pool.get().unwrap();

        let diagnoses = diagnoses_schema::dsl::diagnoses
            .load::<Diagnoses>(&conn)
            .unwrap();

        let diagnoses_entities = diagnoses
            .into_iter()
            .map(|diagnosis| to_entity(diagnosis))
            .collect();

        return diagnoses_entities;
    }

    fn store(
        &self,
        diagnosis: &diagnosis::Diagnosis,
    ) -> Result<(), String> {
        let conn = self.pool.get().unwrap();

        let diagnosis_model = Diagnoses {
            id: diagnosis.id().value(),
            title: diagnosis.title().value(),
            organization_id: diagnosis.organization_id().value(),
        };

        let res_diagnosis =
            diesel::insert_into(diagnoses_schema::dsl::diagnoses)
                .values(diagnosis_model)
                .execute(&conn);

        match res_diagnosis {
            Ok(_) => Ok(()),
            Err(err) => Err("Failed to store diagnosis".to_string()),
        }
    }
}
