use super::leader_change_context;
use crate::core::leader_change_context::LeaderChangeContext;

pub trait StateMachine {
    ///Update the StateMachine with a batch a tasks that can be accessed through |iterator|.
    /// Invoked when one or more tasks that were passwd to Node#apply(Task) have been committed to the
    /// raft group (quorum of the group peers have received those tasks and stored them on the backing storage
    ///
    /// Once this function returns to the caller, we will regard all the iterated tasks through |iter| have
    /// been successfully applied. And if you didn't apply all the given tasks, we would regard this as a critical
    /// error and report a error whose type is ERROR_TYPE_STATE_MACHINE.
    ///
    fn on_apply();

    ///Invoked once when the raft node was shut down.
    /// Default do nothing.
    fn on_shutdown();

    ///User defined snapshot generate function. this method will block StateMachine#onApply.
    /// user can make snapshot async when fsm can be cow(copy-on-write)
    /// call done.run(status) when snapshot finished.
    /// Default: Save nothing and returns error.
    fn on_snapshot_save();

    ///Invoked when the belonging node becomes the leader of the group at |term|
    fn on_leader_start(term: u64);

    ///This method is called when a critical error was encountered, after this point,
    /// no any further modification is allowed to applied to this node
    /// util the error is fixed and this node restarts.
    ///
    /// @param e raft error message
    fn on_error();

    ///This method is called when a follower stops following a leader and
    /// its leaderId becomes null,situations including:
    /// 1. handle election timeout and start preVote
    /// 2. receive requests with higher term such as VoteRequest from candidate
    ///      or appendEntries request from a new leader
    /// 3. receive timeoutNow request from current leader and start request vote.
    ///
    /// the parameter ctx gives the information(leaderId, term and status) about the
    /// very leader whom the follower followed before.
    /// User can reset the node's information as it stops following some leader.
    fn on_stop_following(ctx: LeaderChangeContext);

    ///This method is called when a follower or candidate starts following a leader and its leaderId
    /// (should be NULL before the method is called) is set to leader's id,
    /// situations including:
    /// 1. a candidate receives appendEntries request from a leader
    /// 2. a follower(without leader) receives appendEntries from a leader
    ///
    /// the parameter ctx gives the information(leaderId, term and status) about
    /// the very leader whom the follower starts to follow.
    /// User can reset the node's information as it starts to follow some leader.
    ///
    /// @param ctx context of leader change
    fn on_start_following(ctx: LeaderChangeContext);
}
