use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Application {
    pub name: String
}

impl Application {
    pub fn new(name: &str) -> Self {
        Application {
            name: String::from(name)
        }
    }
}
