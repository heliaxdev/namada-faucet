use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum NodeType {
    Validator,
    Seed,
    Full,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Node {
    pub host: String,
    pub node_type: NodeType,
}