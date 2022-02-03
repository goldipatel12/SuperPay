use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StreamU8 {
    pub data: Vec<u8>,
}
