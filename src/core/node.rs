use super::node_id;
use super::node_id::NodeId;
use super::node_options::NodeOptions;
use super::peer_id::PeerId;
use super::raft_options::RaftOptions;
use super::task::Task;
use std::collections::LinkedList;

pub trait node {
    ///Get the leader peer id for redirect,null if absent
    fn get_leader_id() -> Option<PeerId>;
    ///Get current node id
    fn get_node_id() -> NodeId;
    ///Get the raft group id
    fn get_group_id() -> String;
    ///Get the node options
    fn get_options() -> NodeOptions;
    ///Get the raft options
    fn get_raft_options() -> RaftOptions;
    ///Returns true when the node is leader.
    fn is_leader() -> bool;
    ///Shutdown local replica node
    ///@param done callback
    fn shutdown();
    /// Block the thread until the node is successfully stopped
    fn join();
    ///Apply task to the replicated-state-machine
    fn apply(task: Task);
    /// Starts a linearizable read-only query request with request context(optional,
    /// such as request id etc.) and closure.  The closure will be called when the
    /// request is completed, and user can read data from state machine if the result
    /// status is OK.
    fn read_index();

    ///List peers of this raft group, only leader returns
    fn list_peers() -> LinkedList<PeerId>;

    ///List all alive peers of this raft group
    fn list_alive_peers() -> LinkedList<PeerId>;
    ///Add a new peer to the raft group
    fn add_peer(peer: PeerId);
    ///Remove the peer from the raft group
    fn remove_peer(peer: PeerId);
    ///Start a snapshot immediately if possible
    fn snapshot();
    ///Reset the election_timeout for the every node
    fn reset_election_timeout_ms(timeout_ms: u64);
    ///Try to transferring leadership to peer.
    fn transfer_leader_ship_to(peer: PeerId);
    ///Read the first committed user log from the given index.
    fn read_committed_user_log(index: u64);
}
