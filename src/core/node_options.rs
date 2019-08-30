pub struct NodeOptions {
    /// A follower would become a candidate if it doesn't receive any message
    /// from the leader in |election_timeout_ms| milliseconds
    /// Default: 1000(1s)
    pub election_timeout_ms: i32,
    /// Leader lease time's ratio of election_timeout_ms,
    /// To minimize the effects of clock drift, we should make that:
    /// clockDrift + leaderLeaseTimeoutMs < electionTimeout
    pub leader_lease_time_ratio: i32,
    /// A snapshot saving would be triggered every |snapshot_interval_s| seconds
    /// if this was reset as positive number
    /// If |snapshot_interval_s| <= 0, the time based snapshot would be disabled
    ///
    /// Default: 3600(1 hour)
    pub snapshot_interval_secs: i32,
}

impl Default for NodeOptions {
    fn default() -> Self {
        NodeOptions {
            election_timeout_ms: 1000,
            leader_lease_time_ratio: 90,
            snapshot_interval_secs: 3600,
        }
    }
}

impl NodeOptions {
    pub fn new() -> NodeOptions {
        Default::default()
    }
}
