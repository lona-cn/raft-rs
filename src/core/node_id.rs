use crate::core::peer_id::PeerId;

pub struct NodeId {
    ///Raft group id
    group_id: String,
    ///Node peer id
    peer_id: PeerId,
}

impl NodeId {
    pub fn new(group_id: String, peer_id: PeerId) -> NodeId {
        NodeId { group_id, peer_id }
    }
}
