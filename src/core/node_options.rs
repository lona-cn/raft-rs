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
}
