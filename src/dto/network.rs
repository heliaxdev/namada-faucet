use serde::{Deserialize, Serialize};

use super::node::Node;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NetworkListResponse {
    pub networks: Vec<NetworkWithId>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct NetworkWithId {
    pub network: Network,
    pub network_id: String
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct Network {
    pub chain_id: ChainId,
    pub nodes: Vec<Node>,
    pub network_type: NetworkType,
    pub status: NetworkStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum ChainId {
    #[default]
    Uninitialized,
    Id(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
pub enum NetworkStatus {
    #[default]
    Uninitialized,
    Setup,
    SetupSpawnMachines,
    SetupArtifacts,
    SetupMachines,
    Online,
    ToBeDeleted,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, Eq, PartialEq)]
pub enum NetworkType {
    #[default]
    InternalDevnet,
    PublicDevnet,
    Testnet,
}