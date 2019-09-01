pub struct Status {}

#[derive(PartialOrd, PartialEq)]
pub enum State {
    ///It's a leader
    State_Leader,
    ///It's transferring leadership
    State_Transferring,
    ///It's a candidate
    State_Candidate,
    ///It's a follower
    State_Follower,
    ///It's in error
    State_Error,
    ///It's uninitialized
    State_Uninitialized,
    ///It's shutting down
    State_Shutting,
    ///It's shutdown already
    State_Shutdown,
    ///State end
    State_End,
}

pub fn is_active(state: State) -> bool {
    return state < State::State_Error;
}
