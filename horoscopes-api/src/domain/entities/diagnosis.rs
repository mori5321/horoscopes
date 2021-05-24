use super::organization::ID as OrganizationID;

#[derive(Clone, Debug)]
pub struct Diagnosis {
    id: ID,
    title: Title,
    organization_id: OrganizationID,
}

impl Diagnosis {
    pub fn new(
        id: String,
        title: String,
        organization_id: String,
    ) -> Self {
        Self {
            id: ID::new(id),
            title: Title::new(title),
            organization_id: OrganizationID::new(organization_id),
        }
    }

    pub fn id(&self) -> ID {
        self.id.clone()
    }

    pub fn title(&self) -> Title {
        self.title.clone()
    }

    pub fn organization_id(&self) -> OrganizationID {
        self.organization_id.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ID(String);

impl ID {
    pub fn new(id: String) -> Self {
        ID(id)
    }

    pub fn value(&self) -> String {
        let ID(id) = self;
        id.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title(String);

impl Title {
    fn new(title: String) -> Self {
        Title(title)
    }

    pub fn value(&self) -> String {
        let Title(title) = self;
        title.clone()
    }
}
