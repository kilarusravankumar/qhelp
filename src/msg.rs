use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub role: String,
    pub content: String,
}
