use crate::ocnode::Ocnode;
use serde::{Deserialize, Serialize};

/// Used to serialize a scene.
#[derive(Serialize, Deserialize)]
pub struct StoredOctree {
    pub active_nodes: Vec<Ocnode>,
}
