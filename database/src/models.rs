pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Epic { stories: vec![] }
    }
}

pub struct Story {
    pub id: u32,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Story {
            id: 0,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}
