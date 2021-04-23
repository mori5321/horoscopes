use ulid::Ulid;
use crate::usecases::common::ports::providers::IDProvider;

pub struct ULIDProvider {}

impl ULIDProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl IDProvider for ULIDProvider {
    fn generate(&self) -> String {
        let ulid  = Ulid::new();
        ulid.to_string()
    }
}
