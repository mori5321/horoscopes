use crate::db::MysqlPool;
use crate::domain::entities::diagnosis;
use crate::domain::repositories::DiagnosisRepository;
use crate::models::diagnoses::Diagnoses;
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
    fn store(
        &self,
        diagnosis: &diagnosis::Diagnosis,
    ) -> Result<(), String> {
        let conn = self.pool.get().map_err(|err| {
            "Failed to get database connection from pool.".to_string()
        })?;

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
