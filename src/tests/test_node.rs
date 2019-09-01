use crate::core::node_options::NodeOptions;
#[test]
fn start_node() {
    let node_options = NodeOptions::new();
    assert_eq!(node_options.get_election_timeout_ms(), 1000);
    assert_eq!(node_options.get_leader_lease_time_ratio(), 90);
    assert_eq!(node_options.get_snapshot_interval_secs(), 3600);
}

#[macro_use]
use crate::core::logger;
use std::{thread, time};
#[test]
fn test_logger() {
    assert_eq!(logger::init(logger::LoggingLevel::Debug), true);
    launch_info!("Hello logger");
    launch_info!("Hello World");
    debug_!("Hello, Debug");
}
