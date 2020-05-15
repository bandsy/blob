use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub file_name: String
}
