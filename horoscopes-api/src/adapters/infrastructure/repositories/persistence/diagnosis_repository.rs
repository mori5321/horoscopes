use crate::db::MysqlPool;
use crate::domain::entities::diagnosis::{self, Diagnosis};
use crate::domain::repositories::DiagnosisRepository;
use crate::models::diagnoses::{to_entity, Diagnoses};
use crate::schema::diagnoses as diagnoses_schema;

use diesel::prelude::*;
use std::sync::Arc;

pub struct DiagnosisRepositoryImpl {
    pool: Arc<MysqlPool>,
}

impl DiagnosisRepositoryImpl {
    pub fn new(pool: Arc<MysqlPool>) -> Self {
        Self { pool }
    }
}

impl DiagnosisRepository for DiagnosisRepositoryImpl {
    fn list(&self) -> Option<Vec<Diagnosis>> {
        let conn = self.pool.get().unwrap();

        let opt_diagnoses = diagnoses_schema::dsl::diagnoses
            .load::<Diagnoses>(&conn)
            .ok();

        let opt_diagnoses_entities = opt_diagnoses.map(|diagnoses| {
            return diagnoses
                .into_iter()
                .map(|diagnosis| to_entity(diagnosis))
                .collect();
        });

        return opt_diagnoses_entities;
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
