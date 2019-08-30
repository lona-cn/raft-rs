use crate::core::peer_id::PeerId;
use crate::core::status::Status;

pub struct LeaderChangeContext {
    leader_id: PeerId,
    term: u64,
    status: Status,
}