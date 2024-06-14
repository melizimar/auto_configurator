use crate::core::entities::application_entity::Application;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApplicationDTO {
    pub name: String
}

impl From<ApplicationDTO> for Application {
    fn from(dto: ApplicationDTO) -> Application {
        Application {
            name: dto.name
        }
    }
}