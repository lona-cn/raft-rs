use crate::core::node_options::NodeOptions;

#[test]
fn start_node() {
    let node_options = NodeOptions::new();
    assert_eq!(node_options.get_election_timeout_ms(), 1000);
    assert_eq!(node_options.get_leader_lease_time_ratio(), 90);
    assert_eq!(node_options.get_snapshot_interval_secs(), 3600);
}
