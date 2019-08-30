use crate::core::state_machine::StateMachine;

pub struct NodeOptions {
    /// A follower would become a candidate if it doesn't receive any message
    /// from the leader in |election_timeout_ms| milliseconds
    /// Default: 1000(1s)
    election_timeout_ms: i32,
    /// Leader lease time's ratio of election_timeout_ms,
    /// To minimize the effects of clock drift, we should make that:
    /// clockDrift + leaderLeaseTimeoutMs < electionTimeout
    leader_lease_time_ratio: i32,
    /// A snapshot saving would be triggered every |snapshot_interval_s| seconds
    /// if this was reset as positive number
    /// If |snapshot_interval_s| <= 0, the time based snapshot would be disabled
    ///
    /// Default: 3600(1 hour)
    snapshot_interval_secs: i32,
    ///If true, RPCs through raft_cli will be denied
    /// Default: false
    disable_cli: bool,

    state_machine: Option<Box<dyn StateMachine>>,
}

impl Default for NodeOptions {
    fn default() -> Self {
        NodeOptions {
            election_timeout_ms: 1000,
            leader_lease_time_ratio: 90,
            snapshot_interval_secs: 3600,
            disable_cli: false,
            state_machine: None,
        }
    }
}

impl NodeOptions {
    pub fn new() -> NodeOptions {
        Default::default()
    }

    pub fn get_election_timeout_ms(&self) -> i32 {
        self.election_timeout_ms
    }

    pub fn set_election_timeout_ms(&mut self, election_timeout_ms: i32) {
        self.election_timeout_ms = election_timeout_ms;
    }

    pub fn get_leader_lease_time_ratio(&self) -> i32 {
        self.leader_lease_time_ratio
    }

    pub fn set_leader_lease_time_ratio(&mut self, leader_lease_time_ratio: i32) {
        self.leader_lease_time_ratio = leader_lease_time_ratio;
    }

    pub fn get_snapshot_interval_secs(&self) -> i32 {
        self.snapshot_interval_secs
    }

    pub fn set_snapshot_interval_secs(&mut self, snapshot_interval_secs: i32) {
        self.snapshot_interval_secs = snapshot_interval_secs;
    }

    pub fn get_disable_cli(&self) -> bool {
        self.disable_cli
    }
    pub fn set_disable_cli(&mut self, disable_cli: bool) {
        self.disable_cli = disable_cli;
    }
}
